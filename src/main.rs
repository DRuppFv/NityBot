mod commands;
mod handler;
mod utils;
mod groups;
mod primitives;

use handler::Handler;
use utils::handle_result;
use groups::GENERAL_GROUP;

use std::collections::HashSet;
use std::env;

use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::*;
use serenity::Client;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
  //dotenv & token
  match dotenv() {
    Ok(x) => x,
    Err(err) => panic!("dotenv: {:?}", err)
  };

  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  let http = Http::new(&token);

  //database
  let database = sqlx::sqlite::SqlitePoolOptions::new()
  .connect_with(
      sqlx::sqlite::SqliteConnectOptions::new().filename("languages.db")
  ).await.expect("");

  sqlx::migrate!("./migrations").run(&database).await.expect("");

  //client & framework
  let (owners, bot_id) = match http.get_current_application_info().await {
    Ok(info) => {
        let mut owners = HashSet::new();
        owners.insert(info.owner.id);

        match http.get_current_user().await {
            Ok(bot_id) => (owners, bot_id.id),
            Err(why) => exit!(1, "Could not access the bot id: {:?}", why),
        }
    }
    Err(why) => exit!(2, "No app info:\n{:?}", why),
};


let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .on_mention(Some(bot_id))
                   .prefix("!f")
                   .delimiters(vec![", ", ",", " "])
                   .owners(owners))
        .after(handle_result)
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