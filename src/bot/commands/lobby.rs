use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::{
    application_command::ApplicationCommandInteraction,
    InteractionResponseType
};
use serenity::async_trait;
use serenity::http::CacheHttp;
use serenity::model::application::command::CommandOptionType;
use serenity::model::channel::{ChannelType, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::model::Permissions;
use crate::bot::commands::MixerCommand;
use crate::database::DatabaseContainer;
use crate::mixer::mixer;
use crate::mixer::player::Player;
use crate::mixer::role::Role;


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
        let member = guild_id.member(ctx.http(), interaction.user.id).await?;
        let mut has_permission = false;
        if let Ok(perms) =  member.permissions(&ctx.cache) {
            if perms.manage_channels() {
                has_permission = true;
            }
        }

        if !has_permission {
            interaction.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("You don't have permission to create a lobby!")
                    })
            }).await?;
            return Ok(())
        }

        let data = ctx.data.read().await;
        let db = data.get::<DatabaseContainer>().unwrap().read().await;

        let main_voice = interaction.guild_id.unwrap().create_channel(&ctx.http, |c| {
            c.name("Mix Lobby").kind(ChannelType::Voice)
        }).await?;

        let permissions = vec![
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(RoleId::from(interaction.guild_id.unwrap().0))
            }
        ];
        let red_voice = interaction.guild_id.unwrap().create_channel(&ctx.http, |c| {
            c.name("Red").kind(ChannelType::Voice).user_limit(5).permissions(permissions.clone())
        }).await?;
        let blue_voice = interaction.guild_id.unwrap().create_channel(&ctx.http, |c| {
            c.name("Blue").kind(ChannelType::Voice).user_limit(5).permissions(permissions)
        }).await?;

        db.insert_guild_lobby(interaction.guild_id.unwrap(), main_voice.id, red_voice.id, blue_voice.id).await;

        interaction.create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Successfully created a new mix lobby!")
                })
        }).await?;

        Ok(())
    }

    async fn start_lobby(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()> {
        let data = ctx.data.read().await;
        let db = data.get::<DatabaseContainer>().unwrap().write().await;

        let guild_id = interaction.guild_id.unwrap();
        let member = guild_id.member(ctx.http(), interaction.user.id).await?;

        let mut is_in_lobby = false;
        let mut channel_id = None;
        for (id, channel) in guild_id.channels(ctx.http()).await? {
            if channel.kind != ChannelType::Voice {
                continue;
            }
            let members = channel.members(&ctx.cache).await?;
            if members.iter().any(|m| m.user.id == member.user.id) && db.get_lobby_by_channel(guild_id, id).await.is_some() {
                is_in_lobby = true;
                channel_id = Some(id);
                break;
            }
        }

        if !is_in_lobby {
            interaction.create_interaction_response(ctx.http(), |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("You are not in the mix lobby!").ephemeral(true)
                    })
            }).await?;

            return Ok(());
        }

        let lobby = db.get_lobby_by_channel(guild_id, channel_id.unwrap()).await.unwrap();

        let main_channel = ChannelId::from(lobby.main_voice_id as u64).to_channel(ctx.http()).await.unwrap().guild().unwrap();
        let red_channel = ChannelId::from(lobby.red_team_voice_id as u64).to_channel(ctx.http()).await.unwrap().guild().unwrap();
        let blue_channel = ChannelId::from(lobby.blue_team_voice_id as u64).to_channel(ctx.http()).await.unwrap().guild().unwrap();

        for member in red_channel.members(ctx.cache().unwrap()).await? {
            member.move_to_voice_channel(ctx.http(), main_channel.id).await?;
        }
        for member in blue_channel.members(ctx.cache().unwrap()).await? {
            member.move_to_voice_channel(ctx.http(), main_channel.id).await?;
        }

        // TODO: uncomment this
        // let members = main_channel.members(ctx.cache().unwrap()).await?;
        // let users = members.iter().map(|m| m.user.id).collect::<Vec<UserId>>();
        let users = (0..10).map(|id| UserId::from(id)).collect::<Vec<UserId>>();
        let players = db.get_players(users).await;
        let players = players.into_iter().map(|p| Player::new(p)).collect::<Vec<Player>>();
        let slots = vec![Role::Tank, Role::Dps, Role::Dps, Role::Support, Role::Support];
        if let Some((team1, team2)) = mixer::mix_players(&players, slots) {
            println!("Average rank {}", team1.average_rank());
            println!("Average rank tank {}", team1.average_rank_role(&Role::Tank));
            println!("Average rank dps {}", team1.average_rank_role(&Role::Dps));
            println!("Average rank support {}", team1.average_rank_role(&Role::Support));
            println!("Average rank {}", team2.average_rank());
            println!("Average rank tank {}", team2.average_rank_role(&Role::Tank));
            println!("Average rank dps {}", team2.average_rank_role(&Role::Dps));
            println!("Average rank support {}\n", team2.average_rank_role(&Role::Support));
            println!("Team 1: {:?}\n\n", team1.players.iter().map(|p| (p.0.clone().0, p.1.clone().unwrap().name.clone())).collect::<Vec<(Role, String)>>());
            println!("Team 2: {:?}", team2.players.iter().map(|p| (p.0.clone().0, p.1.clone().unwrap().name.clone())).collect::<Vec<(Role, String)>>());
        }
        else {
            println!("Fair lobby could not be mixed")
        }

        interaction.create_interaction_response(ctx.http(), |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Successfully started the mix lobby!").ephemeral(true)
                })
        }).await?;

        Ok(())
    }
}