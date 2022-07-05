use serenity::framework::standard::{macros::command, CommandResult};
use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::model::channel::Message;

#[command]
async fn wiki(ctx: &Context, msg: &Message, arg: Args) -> CommandResult {
    
    Ok(())
}