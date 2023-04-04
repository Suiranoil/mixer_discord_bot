pub mod commands;
pub mod interactions;
mod handlers;

use std::collections::HashMap;
use std::sync::Arc;
use serenity::{CacheAndHttp, Client};
use serenity::prelude::{GatewayIntents, TypeMap, TypeMapKey};
use tokio::sync::RwLock;
use crate::bot::commands::MixerCommand;
use crate::database::{MixerDatabase, DatabaseContainer};
use crate::bot::handlers::command_handler::{MixerCommandHandler, MixerCommandHandlerContainer};
use crate::bot::handlers::event_handler::Handler;

pub struct MixerBot {
    token: String,
    commands: Option<HashMap<String, Box<dyn MixerCommand>>>,
    // lobbies: Vec<Lobby>
}

struct MixerBotContainer;


impl TypeMapKey for MixerBotContainer {
    type Value = Arc<RwLock<MixerBot>>;
}


impl MixerBot {
    pub fn new(token: String) -> Self {
        Self {
            token,
            commands: Some(HashMap::new()),
            // lobbies: vec![]
        }
    }

    pub async fn start(mut self) -> serenity::Result<()> {
        let mut client = Client::builder(&self.token, GatewayIntents::all()).event_handler(Handler).await?;

        {
            let mut data = client.data.write().await;
            data.insert::<MixerCommandHandlerContainer>(Arc::new(MixerCommandHandler::new(self.commands.unwrap())));
            self.commands = None;

            data.insert::<MixerBotContainer>(Arc::new(RwLock::new(self)));

            let db = MixerDatabase::new("sqlite://database/data.db?mode=rwc").await;
            db.init("./database/script.sql").await;
            data.insert::<DatabaseContainer>(Arc::new(RwLock::new(db)));
        }

        let shard_manager = client.shard_manager.clone();
        let cache_and_http = client.cache_and_http.clone();
        let data = client.data.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");

            let data_ = data.clone();
            let data_ = data_.read().await;
            let bot = data_.get::<MixerBotContainer>().unwrap();
            bot.write().await.shutdown(data, cache_and_http).await;

            shard_manager.lock().await.shutdown_all().await;
        });

        client.start().await?;

        Ok(())
    }

    pub fn add_command<T: MixerCommand + 'static>(&mut self, command: T) -> &mut Self {
        self.commands.as_mut().unwrap().insert(command.name(), Box::new(command));
        self
    }

    pub async fn shutdown(&mut self, data: Arc<RwLock<TypeMap>>, cache_and_http: Arc<CacheAndHttp>) {

        println!("Bot has been shutdown.");
    }
}
