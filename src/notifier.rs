use serde::Deserialize;
use serenity::{
    all::{ChannelId, Context, CreateMessage, EventHandler, GatewayIntents, Ready},
    async_trait, Client,
};
use std::error::Error;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct DiscordNotifier {
    token: String,
    channel_id: String,
}

impl DiscordNotifier {
    pub async fn send(&self, message: String) -> Result<(), Box<dyn Error>> {
        let handler = Handler {
            channel_id: self.channel_id.clone(),
            message,
        };

        let mut client = Client::builder(&self.token, GatewayIntents::empty())
            .event_handler(handler)
            .await?;

        // 10秒後にclientをシャットダウン
        let shard_manager = client.shard_manager.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            shard_manager.shutdown_all().await;
        });

        client.start().await?;
        Ok(())
    }
}

struct Handler {
    channel_id: String,
    message: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let channel_id = ChannelId::from_str(&self.channel_id).expect("Failed to parse channel id");
        channel_id
            .send_message(&ctx.http, CreateMessage::new().content(&self.message))
            .await
            .expect("Failed to send message");
    }
}
