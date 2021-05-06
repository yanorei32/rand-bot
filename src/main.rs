use std::env;
use rand::seq::IteratorRandom;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "[rand-bot] pong").await {
                println!("Error sending message: {:?}", why);
            }

            return
        }

        if msg.content.starts_with("!rand") {
            let command = msg.content.trim_end().split(' ').skip(1);
            let argc = command.clone().count();

            if argc == 0 {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Invalid argument count (0).").await {
                    println!("Error sending message: {:?}", why);
                }

                return
            }

            let resp = command.choose(&mut rand::thread_rng()).unwrap().replace("@", "[at]");

            if let Err(why) = msg.channel_id.say(&ctx.http, format!("[rand-bot] {}", resp)).await {
                println!("Error sending message: {:?}", why);
            }

            return
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
