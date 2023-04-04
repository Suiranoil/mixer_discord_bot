mod bot;
mod mixer;
mod database;

use crate::bot::commands::lobby::LobbyCommand;
use crate::bot::commands::ping::PingCommand;
use crate::bot::commands::preference::PreferenceCommand;
use crate::bot::commands::rank::RankCommand;
use crate::bot::MixerBot;

#[tokio::main]
async fn main() -> serenity::Result<()> {
    let mut bot = MixerBot::new(
        "NTE2MzMyMzM2NzQ5NzQwMDUz.GiLPzQ.j5gIUGqx6vF6CFhJv8yizksDi-dOBqCvxR32EE".to_string()
    );

    bot.add_command(PingCommand);
    bot.add_command(LobbyCommand);
    bot.add_command(RankCommand);
    bot.add_command(PreferenceCommand);

    bot.start().await?;

    Ok(())
}
