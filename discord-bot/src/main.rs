
use crate::bot::discord::DiscordBot;
pub mod bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let discord_bot = DiscordBot::new()?;
    discord_bot.login().await?;


    Ok(())
}
