mod bot;
mod mixer;
mod database;
mod algorithm;

use std::sync::Arc;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use shuttle_runtime::Context;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use tokio::sync::RwLock;
use crate::bot::commands::lobby::LobbyCommand;
use crate::bot::commands::ping::PingCommand;
use crate::bot::commands::preference::PreferenceCommand;
use crate::bot::commands::rank::RankCommand;
use crate::bot::commands::settings::SettingsCommand;
use crate::bot::MixerBot;
use crate::database::{DatabaseContainer, MixerDatabase};


#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store.get("DISCORD_TOKEN").context("'DISCORD_TOKEN' was not found")?;
    let app_id = secret_store.get("DISCORD_APP_ID").context("'DISCORD_APP_ID' was not found")?;

    pool.execute(include_str!("../schema.sql")).await.unwrap();

    let mut bot = MixerBot::new();

    bot.add_command(PingCommand);
    bot.add_command(LobbyCommand);
    bot.add_command(RankCommand);
    bot.add_command(PreferenceCommand);
    bot.add_command(SettingsCommand);

    let client = Client::builder(&token, GatewayIntents::all())
        .event_handler(bot)
        .application_id(app_id.parse::<u64>().unwrap())
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        let db = MixerDatabase::new(pool);
        data.insert::<DatabaseContainer>(Arc::new(RwLock::new(db)));
    }

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");

        println!("Bot has been shutdown.");

        shard_manager.lock().await.shutdown_all().await;
    });

    Ok(client.into())
}
