use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use reqwest;
use std::env;

#[derive(Debug)]
#[warn(dead_code)]
pub struct DiscordBot {
    app_id: String,
    discord_token: String,
    public_key: String,
    client: reqwest::Client,
    approved: Arc<Mutex<bool>>,
}

impl DiscordBot {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let app_id = env::var("DISCORD_APP_ID").expect("DISCORD_APP_ID not found in environment variables.");        
        let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in environment variables.");          
        let public_key = env::var("DISCORD_PUBLIC_KEY").expect("DISCORD_PUBLIC_KEY not found in environment variables.");          
        let client = reqwest::Client::new();
        let approved = Arc::new(Mutex::new(false));
        Ok(Self {
            app_id,
            discord_token,
            public_key, client, approved
        })
    }

    pub async fn login(&self) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.client.get("https://discord.com/api/v10/users/@me")
        .header("Authorization", format!("Bot {}", self.discord_token))
        .send()
        .await?;

        println!("Response: {:?}\n", result);
        Ok(())

    } 

    pub fn is_approved(&self) -> bool {
        *self.approved.lock().unwrap()
    }
}
