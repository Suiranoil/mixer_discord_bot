pub mod commands;
mod handler;

use std::collections::HashMap;
use std::sync::Arc;
use serenity::{CacheAndHttp, Client};
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::{GatewayIntents, TypeMap, TypeMapKey};
use tokio::sync::{Mutex, RwLock};
use crate::bot::commands::MixerCommand;
use crate::bot::handler::Handler;

pub struct MixerBot {
    token: String,
    commands: HashMap<String, Box<dyn MixerCommand>>,
}

struct ShardManagerContainer;
struct MixerBotContainer;


impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for MixerBotContainer {
    type Value = Arc<RwLock<MixerBot>>;
}


impl MixerBot {
    pub fn new(token: String) -> Self {
        Self {
            token,
            commands: HashMap::new(),
        }
    }

    pub async fn start(self) -> serenity::Result<()> {
        let mut client = Client::builder(&self.token, GatewayIntents::empty()).event_handler(Handler).await?;

        let bot;
        {
            let mut data = client.data.write().await;
            data.insert::<ShardManagerContainer>(client.shard_manager.clone());
            data.insert::<MixerBotContainer>(Arc::new(RwLock::new(self)));
            bot = data.get::<MixerBotContainer>().unwrap().clone();
        }

        let shard_manager = client.shard_manager.clone();
        let cache_and_http = client.cache_and_http.clone();
        let data = client.data.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");

            bot.write().await.shutdown(data, cache_and_http).await;

            shard_manager.lock().await.shutdown_all().await;
        });


        client.start().await?;

        Ok(())
    }

    pub fn add_command(&mut self, command: Box<dyn MixerCommand>) -> &mut Self {
        self.commands.insert(command.name(), command);
        self
    }

    pub async fn shutdown(&self, data: Arc<RwLock<TypeMap>>, cache_and_http: Arc<CacheAndHttp>) {
        println!("{:#?}", cache_and_http.http);
        println!("Bot has been shutdown.");
    }
}