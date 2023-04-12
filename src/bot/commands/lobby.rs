use std::time::Duration;
use itertools::Itertools;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::interaction::{
    application_command::ApplicationCommandInteraction,
    InteractionResponseType
};
use serenity::async_trait;
use serenity::futures::future::join_all;
use serenity::futures::StreamExt;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::model::Permissions;
use serenity::utils::Colour;
use crate::bot::commands::MixerCommand;
use crate::database::DatabaseContainer;
use crate::database::models::lobby::Model;
use crate::mixer::mixer;
use crate::mixer::player::Player;
use crate::mixer::role::Role;
use crate::mixer::team::Team;


#[derive(Clone)]
pub struct LobbyCommand;

#[async_trait]
impl MixerCommand for LobbyCommand {
    fn name(&self) -> String {
        "lobby".to_string()
    }

    fn create(&self, command: &mut CreateApplicationCommand) {
        command.name(self.name()).description("Create or start a lobby")
            .create_option(|option| {
                option.name("create")
                    .description("Create a lobby")
                    .kind(CommandOptionType::SubCommand)
            })
            .create_option(|option| {
                option.name("start")
                    .description("Start a lobby")
                    .kind(CommandOptionType::SubCommand)
            })
            .default_member_permissions(Permissions::MOVE_MEMBERS)
            .dm_permission(false);
    }

    async fn execute(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()> {
        match interaction.data.options.get(0).unwrap().name.as_str() {
            "create" => self.create_lobby(ctx, interaction).await,
            "start" => self.start_lobby(ctx, interaction).await,
            _ => Ok(())
        }
    }
}

impl LobbyCommand {
    async fn create_lobby(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()> {
        let guild_id = interaction.guild_id.unwrap();
        let guild = guild_id.to_partial_guild(ctx).await?;
        let member = guild.member(ctx, interaction.user.id).await?;

        let mut has_permission = false;
        if guild.owner_id == member.user.id {
            has_permission = true;
        }
        if let Ok(perms) =  member.permissions(ctx) {
            if perms.administrator() {
                has_permission = true;
            }
        }

        if !has_permission {
            interaction.create_interaction_response(ctx, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("You don't have permission to create a lobby!")
                    })
            }).await?;
            return Ok(())
        }

        let data = ctx.data.read().await;
        let db = data.get::<DatabaseContainer>().unwrap().read().await;

        let main_voice = interaction.guild_id.unwrap().create_channel(ctx, |c| {
            c.name("Mix Lobby").kind(ChannelType::Voice)
        }).await?;

        let permissions = vec![
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(RoleId::from(interaction.guild_id.unwrap().0))
            }
        ];
        let red_voice = interaction.guild_id.unwrap().create_channel(ctx, |c| {
            c.name("Red").kind(ChannelType::Voice).user_limit(5).permissions(permissions.clone())
        }).await?;
        let blue_voice = interaction.guild_id.unwrap().create_channel(ctx, |c| {
            c.name("Blue").kind(ChannelType::Voice).user_limit(5).permissions(permissions)
        }).await?;

        db.insert_guild_lobby(interaction.guild_id.unwrap(), main_voice.id, red_voice.id, blue_voice.id).await;
        drop(db);
        drop(data);

        interaction.create_interaction_response(ctx, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Successfully created a new mix lobby!")
                })
        }).await?;

        Ok(())
    }

    async fn start_lobby(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()> {
        let data = ctx.data.read().await;
        let db = data.get::<DatabaseContainer>().unwrap().read().await;

        let guild_id = interaction.guild_id.unwrap();
        let member = guild_id.member(ctx, interaction.user.id).await?;

        let channels = guild_id.channels(ctx).await?;

        let mut is_in_lobby = false;
        let mut channel_id = None;
        for (id, channel) in channels {
            if channel.kind != ChannelType::Voice {
                continue;
            }
            let members = channel.members(ctx).await?;
            if members.iter().any(|m| m.user.id == member.user.id) && db.get_lobby_by_channel(guild_id, id).await.is_some() {
                is_in_lobby = true;
                channel_id = Some(id);
                break;
            }
        }

        if !is_in_lobby {
            interaction.create_interaction_response(ctx, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("You are not in the mix lobby!").ephemeral(true)
                    })
            }).await?;

            return Ok(());
        }

        let lobby = db.get_lobby_by_channel(guild_id, channel_id.unwrap()).await.unwrap();

        let main_channel = ChannelId::from(lobby.main_voice_id as u64).to_channel(ctx).await.unwrap().guild().unwrap();
        let red_channel = ChannelId::from(lobby.red_team_voice_id as u64).to_channel(ctx).await.unwrap().guild().unwrap();
        let blue_channel = ChannelId::from(lobby.blue_team_voice_id as u64).to_channel(ctx).await.unwrap().guild().unwrap();

        interaction.create_interaction_response(ctx, |response| {
            response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        }).await?;

        for member in red_channel.members(ctx).await? {
            member.move_to_voice_channel(ctx, main_channel.id).await?;
        }
        for member in blue_channel.members(ctx).await? {
            member.move_to_voice_channel(ctx, main_channel.id).await?;
        }

        // TODO: uncomment this
        let members = main_channel.members(ctx).await?;
        let users = members.iter().map(|m| m.user.id).collect::<Vec<UserId>>();
        // let users = (0..11).map(|id| UserId::from(id)).chain(users.into_iter()).collect::<Vec<UserId>>();
        let players = db.get_players(users).await;
        let players = players.into_iter().map(|p| Player::new(p)).collect::<Vec<Player>>();
        let slots = vec![Role::Tank, Role::Dps, Role::Dps, Role::Support, Role::Support];

        interaction.edit_original_interaction_response(ctx, |response| {
            response.content("Mixing teams...")
        }).await?;

        if let Some(teams) = mixer::mix_players(&players, slots) {
            // let interaction = interaction.clone();
            self.process_valid_teams(ctx, interaction, lobby, teams, players).await?;
        }
        else {
            interaction.edit_original_interaction_response(ctx, |response| {
                response.content("Fair teams' composition could not be found!")
            }).await?;
        }

        Ok(())
    }

    async fn process_valid_teams(&self, ctx: &Context, interaction: ApplicationCommandInteraction, lobby: Model, teams: (Team, Team), players: Vec<Player>) -> serenity::Result<()> {
        let (team1, team2) = teams.clone();

        let team1_names = team1.players.iter()
            .sorted_by(|((a, _), _), ((b, _), _)| i32::from(*a).cmp(&i32::from(*b)))
            .map(|(_, i)| async {
                if let Some(user) = players[i.unwrap()].discord_id.to_user(ctx).await.ok() {
                    user.name
                }
                else {
                    players[i.unwrap()].bn_name.clone()
                }
            }).collect_vec();
        let team2_names = team2.players.iter()
            .sorted_by(|((a, _), _), ((b, _), _)| i32::from(*a).cmp(&i32::from(*b)))
            .map(|(_, i)| async {
                if let Some(user) = players[i.unwrap()].discord_id.to_user(ctx).await.ok() {
                    user.name
                }
                else {
                    players[i.unwrap()].bn_name.clone()
                }
            }).collect_vec();

        let team1_names = join_all(team1_names).await;
        let team2_names = join_all(team2_names).await;
        
        interaction.edit_original_interaction_response(ctx, |response| {
            response.content("").components(|components| {
                components.create_action_row(|row| {
                    row.create_button(|button| {
                        button.custom_id("cancel").label("Cancel")
                            .style(ButtonStyle::Danger)
                    });
                    row.create_button(|button| {
                        button.custom_id("swap").label("Swap").disabled(true)
                            .style(ButtonStyle::Primary)
                    });
                    row.create_button(|button| {
                        button.custom_id("start").label("Start")
                            .style(ButtonStyle::Success)
                    })
                })
            }).embed(|embed| {
                embed.title("Teams").fields(vec![
                    ("Team 1", "", false),
                    ("Tank", &team1_names[0], true),
                    ("Dps", &format!("{}\n{}", team1_names[1], team1_names[2]), true),
                    ("Support", &format!("{}\n{}", team1_names[3], team1_names[4]), true),
                    ("Team 2", "", false),
                    ("Tank", &team2_names[0], true),
                    ("Dps", &format!("{}\n{}", team2_names[1], team2_names[2]), true),
                    ("Support", &format!("{}\n{}", team2_names[3], team2_names[4]), true),
                ]).colour(Colour::new(0xcfa22f))
            })
        }).await.unwrap();

        let msg = interaction.get_interaction_response(ctx).await.unwrap();
        let collector = msg.await_component_interactions(ctx)
            .timeout(Duration::from_secs(2 * 60))
            .guild_id(interaction.guild_id.unwrap())
            .channel_id(interaction.channel_id)
            .author_id(interaction.user.id)
            .collect_limit(1)
            .build();

        let interactions = collector.collect::<Vec<_>>().await;
        if let Some(interaction) = interactions.first() {
            match interaction.data.custom_id.as_str() {
                "start" => self.process_valid_teams_start(ctx, interaction, lobby, &team1, &team2, players).await?,
                "cancel" => self.process_valid_teams_cancel(ctx, interaction, &team1, &team2).await?,
                "swap" => self.process_valid_teams_swap(ctx, interaction, &team1, &team2).await?,
                _ => {}
            }
        }
        else {
            interaction.delete_original_interaction_response(ctx).await?;
        }

        Ok(())
    }

    async fn process_valid_teams_start(&self, ctx: &Context, interaction: &MessageComponentInteraction, lobby: Model, team1: &Team, team2: &Team, players: Vec<Player>) -> serenity::Result<()> {
        let main_channel = ChannelId::from(lobby.main_voice_id as u64).to_channel(ctx).await.unwrap().guild().unwrap();
        let red_channel = ChannelId::from(lobby.red_team_voice_id as u64).to_channel(ctx).await.unwrap().guild().unwrap();
        let blue_channel = ChannelId::from(lobby.blue_team_voice_id as u64).to_channel(ctx).await.unwrap().guild().unwrap();

        for member in main_channel.members(ctx).await? {
            let index = players.iter().position(|p| p.discord_id == member.user.id);

            if team1.players.iter().any(|(_, i)| *i == index && index.is_some()) {
                member.move_to_voice_channel(ctx, red_channel.id).await?;
            }
            else if team2.players.iter().any(|(_, i)| *i == index && index.is_some()) {
                member.move_to_voice_channel(ctx, blue_channel.id).await?;
            }
        }

        let embed = interaction.message.embeds.get(0).unwrap();
        interaction.delete_original_interaction_response(ctx).await?;

        // create msg without interaction
        let msg = interaction.channel_id.send_message(ctx, |message| {
            message.content(format!("<@{}>", interaction.user.id.0))
                .set_embeds(vec![CreateEmbed::from(embed.clone())])
                .components(|components| {
                components.create_action_row(|row| {
                    row.create_button(|button| {
                        button.custom_id("win_team1")
                            .label("Team 1 win")
                            .style(ButtonStyle::Success)
                    }).create_button(|button| {
                        button.custom_id("draw")
                            .label("Draw")
                            .style(ButtonStyle::Secondary)
                    }).create_button(|button| {
                        button.custom_id("win_team2")
                            .label("Team 2 win")
                            .style(ButtonStyle::Success)
                    })
                }).create_action_row(|row| {
                    row.create_button(|button| {
                        button.custom_id("cancel")
                            .label("Cancel game")
                            .style(ButtonStyle::Danger)
                    })
                })
            })
        }).await?;


        // let msg = interaction.edit_original_interaction_response(ctx, |message| {
        //     message.components(|components| {
        //         components.create_action_row(|row| {
        //             row.create_button(|button| {
        //                 button.custom_id("win_team1")
        //                     .label("Team 1 win")
        //                     .style(ButtonStyle::Success)
        //             }).create_button(|button| {
        //                 button.custom_id("draw")
        //                     .label("Draw")
        //                     .style(ButtonStyle::Secondary)
        //             }).create_button(|button| {
        //                 button.custom_id("win_team2")
        //                     .label("Team 2 win")
        //                     .style(ButtonStyle::Success)
        //             })
        //         }).create_action_row(|row| {
        //             row.create_button(|button| {
        //                 button.custom_id("cancel")
        //                     .label("Cancel game")
        //                     .style(ButtonStyle::Danger)
        //             })
        //         })
        //     })
        // }).await?;

        let collector = msg.await_component_interactions(ctx)
            .timeout(Duration::from_secs(30 * 60))
            .guild_id(interaction.guild_id.unwrap())
            .channel_id(interaction.channel_id)
            .author_id(interaction.user.id)
            .collect_limit(1)
            .build();

        let interactions = collector.collect::<Vec<_>>().await;
        if let Some(interaction) = interactions.first() {
            let mut score = 0.5f32;
            match interaction.data.custom_id.as_str() {
                "win_team1" => score = 1.0,
                "draw" => score = 0.5,
                "win_team2" => score = 0.0,
                "cancel" => {
                    return msg.delete(ctx).await;
                    // return interaction.delete_original_interaction_response(ctx).await;
                }
                _ => {}
            }

            let team1_average_rating = team1.average_rating(&players);
            let team2_average_rating = team2.average_rating(&players);

            let team1 = team1.players.clone().into_iter()
                .filter_map(|((role, _), player)|
                    if player.is_some() {
                        Some((role, player.unwrap()))
                    } else {
                        None
                    }
                ).collect::<Vec<_>>();
            let team2 = team2.players.clone().into_iter()
                .filter_map(|((role, _), player)|
                    if player.is_some() {
                        Some((role, player.unwrap()))
                    } else {
                        None
                    }
                ).collect::<Vec<_>>();

            let data = ctx.data.read().await;
            let db = data.get::<DatabaseContainer>().unwrap().read().await;

            for (role, index) in team1 {
                let mut rating = players[index].ranks[&role];
                rating.update(&team2_average_rating, score);
                db.update_player_rank(players[index].discord_id, Some(role), rating).await;
            }

            for (role, index) in team2 {
                let mut rating = players[index].ranks[&role];
                rating.update(&team1_average_rating, 1.0 - score);
                db.update_player_rank(players[index].discord_id, Some(role), rating).await;
            }

            drop(db);
            drop(data);
        }

        msg.delete(ctx).await
    }

    async fn process_valid_teams_cancel(&self, ctx: &Context, interaction: &MessageComponentInteraction, team1: &Team, team2: &Team) -> serenity::Result<()> {
        interaction.delete_original_interaction_response(ctx).await
    }

    async fn process_valid_teams_swap(&self, ctx: &Context, interaction: &MessageComponentInteraction, team1: &Team, team2: &Team) -> serenity::Result<()> {

        Ok(())
    }
}