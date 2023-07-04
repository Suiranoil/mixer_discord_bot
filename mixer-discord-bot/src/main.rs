mod algorithm;
mod bot;
mod database;
mod image_manipulation;
mod mixer;

use bot::commands::creator::CreatorCommand;
use image_manipulation::{ImageGenerator, ImageGeneratorContainer};
use rusttype::Font;
use serenity::model::prelude::UserId;
use serenity::prelude::{GatewayIntents, TypeMapKey};
use serenity::Client;
use shuttle_runtime::Context;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::log::info;

use crate::bot::commands::lobby::LobbyCommand;
use crate::bot::commands::ping::PingCommand;
use crate::bot::commands::preference::PreferenceCommand;
use crate::bot::commands::rating::RatingCommand;
use crate::bot::commands::settings::SettingsCommand;
use crate::bot::MixerBot;
use crate::database::{DatabaseContainer, MixerDatabase};

struct CreatorContainer;

impl TypeMapKey for CreatorContainer {
    type Value = Arc<UserId>;
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_shared_db::Postgres(
        // local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let app_id = secret_store
        .get("DISCORD_APP_ID")
        .context("'DISCORD_APP_ID' was not found")?;

    pool.execute(include_str!("../schema.sql")).await.unwrap();

    let mut bot = MixerBot::new();

    bot.add_command(PingCommand);
    bot.add_command(LobbyCommand);
    bot.add_command(RatingCommand);
    bot.add_command(PreferenceCommand);
    bot.add_command(SettingsCommand);
    bot.add_command(CreatorCommand);

    let client = Client::builder(&token, GatewayIntents::all())
        .event_handler(bot)
        .application_id(app_id.parse::<u64>().unwrap())
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        let db = MixerDatabase::new(pool);
        data.insert::<DatabaseContainer>(Arc::new(RwLock::new(db)));

        let creator = UserId::from(
            secret_store
                .get("CREATOR_ID")
                .context("'CREATOR_ID' was not found")?
                .parse::<u64>()
                .unwrap(),
        );
        data.insert::<CreatorContainer>(Arc::new(creator));

        let image_generator = ImageGenerator {
            player_font: Font::try_from_bytes(include_bytes!(
                "../assets/fonts/big-noodle-too-oblique.ttf"
            ))
            .unwrap(),
            text_font: Font::try_from_bytes(include_bytes!(
                "../assets/fonts/big-noodle-titling.ttf"
            ))
            .unwrap(),
            teams_image: image::load_from_memory(include_bytes!("../assets/images/teams.png"))
                .unwrap()
                .to_rgb8(),
        };
        data.insert::<ImageGeneratorContainer>(Arc::new(image_generator));
    }

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");

        info!("Bot has been shutdown.");

        shard_manager.lock().await.shutdown_all().await;
    });

    Ok(client.into())
}
