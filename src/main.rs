mod commands;

use std::env;

use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::*;
pub struct ShardManagerContainer;
use groups::{GENERAL_GROUP};
mod groups;
use handler::Handler;
mod handler;
#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let http = Http::new(&token);

  dotenv::dotenv().expect("Failed to load .env file");

  let bot_id = match http.get_current_user().await {
    Ok(info) => info.id,
    Err(why) => panic!("Could not access user info: {:?}", why),
};

  let framework = StandardFramework::new()
  .configure(|c| {
      c.with_whitespace(true).on_mention(Some(bot_id)).prefix("~").delimiters(vec![", ", ","])
  })
  .group(&GENERAL_GROUP);

let intents = GatewayIntents::GUILD_MESSAGES
  | GatewayIntents::DIRECT_MESSAGES
  | GatewayIntents::MESSAGE_CONTENT
  | GatewayIntents::GUILD_MESSAGE_REACTIONS;

let mut client = Client::builder(&token, intents)
  .event_handler(Handler)
  .framework(framework)
  .await
  .expect("Err creating client");

if let Err(why) = client.start().await {
  println!("Client error: {:?}", why);
}
}