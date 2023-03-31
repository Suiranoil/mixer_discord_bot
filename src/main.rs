mod bot;

use crate::bot::commands::ping::Ping;
use crate::bot::MixerBot;

#[tokio::main]
async fn main() -> serenity::Result<()> {
    let mut bot = MixerBot::new(
        "NTE2MzMyMzM2NzQ5NzQwMDUz.GiLPzQ.j5gIUGqx6vF6CFhJv8yizksDi-dOBqCvxR32EE".to_string()
    );

    bot.add_command(Box::new(Ping));

    bot.start().await?;

    Ok(())
}
