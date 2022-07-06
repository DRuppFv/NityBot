mod commands;
mod handler;
mod utils;
mod groups;
mod primitives;

use handler::Handler;
use groups::{GENERAL_GROUP};

use std::path::Path;
use std::env;

use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::*;
use serenity::Client;

use dotenv::from_path;

#[tokio::main]
async fn main() {
  let path = Path::new("./.env.token");
  match from_path(path) {
    Ok(x) => x,
    Err(err) => println!("dotenv: {:?}", err)
  };

  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  let http = Http::new(&token);

  let bot_id = match http.get_current_user().await {
    Ok(info) => info.id,
    Err(why) => panic!("Could not access user info: {:?}", why),
};

  let framework = StandardFramework::new()
  .configure(|c| {
      c.with_whitespace(true).on_mention(Some(bot_id)).prefix("!f").delimiters(vec![", ", ","])
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