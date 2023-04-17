use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::Permissions;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use crate::bot::commands::MixerCommand;
use crate::database::models::role::Role;

pub struct SettingsCommand;

#[async_trait]
impl MixerCommand for SettingsCommand {
    fn name(&self) -> String {
        "settings".to_string()
    }

    fn create(&self, command: &mut CreateApplicationCommand) {
        command
            .name(&self.name())
            .description("Change your server settings")
            .create_option(|option| {
                option
                    .name("roles")
                    .kind(CommandOptionType::SubCommandGroup)
                    .description("Change your role settings")
                    .create_sub_option(|sub_option| {
                        sub_option
                            .name("automatic")
                            .kind(CommandOptionType::SubCommand)
                            .description("Automatically assign roles based on your rank")
                    })
            })
            .default_member_permissions(Permissions::ADMINISTRATOR)
            .dm_permission(false);
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
    ) -> serenity::Result<()> {
        let data = interaction.data.options.get(0).unwrap().clone();
        match data.name.as_str() {
            "roles" => {
                self.process_roles_subcommand(ctx, interaction.clone(), data)
                    .await?
            }
            _ => {}
        }

        interaction
            .create_interaction_response(ctx, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("Settings updated"))
            })
            .await?;

        Ok(())
    }
}

impl SettingsCommand {
    async fn process_roles_subcommand(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
        data: CommandDataOption,
    ) -> serenity::Result<()> {
        match data.options.get(0).unwrap().name.as_str() {
            "automatic" => {
                let roles: HashMap<_, _, RandomState> = HashMap::from_iter(
                    [
                        ("support", Role::Support),
                        ("damage", Role::DPS),
                        ("dps", Role::DPS),
                        ("tank", Role::Tank),
                    ]
                    .into_iter(),
                );
                let ranks: HashMap<_, _, RandomState> = HashMap::from_iter(
                    [
                        ("bronze", 0),
                        ("silver", 1),
                        ("gold", 2),
                        ("platinum", 3),
                        ("diamond", 4),
                        ("master", 5),
                        ("grandmaster", 6),
                    ]
                    .into_iter(),
                );

                let guild = interaction
                    .guild_id
                    .unwrap()
                    .to_partial_guild(ctx)
                    .await
                    .unwrap();
                // let guild_roles = guild.roles;
                let mut guild_roles = HashMap::new();
                for (id, guild_role) in guild.roles {
                    for (name, role) in roles.iter() {
                        if !(guild_role.name.to_lowercase().contains(name)) {
                            continue;
                        }

                        for (name, rank) in ranks.iter() {
                            if !(guild_role.name.to_lowercase().contains(name)) {
                                continue;
                            }
                            if !guild_roles.contains_key(&id) {
                                guild_roles.insert(id, (role, rank));
                            } else {
                                let (_, old_rank) = guild_roles.get(&id).unwrap();
                                if rank > *old_rank {
                                    guild_roles.insert(id, (role, rank));
                                }
                            }
                        }
                    }
                }

                println!("{:#?}", guild_roles);
            }
            _ => {}
        }

        Ok(())
    }
}
