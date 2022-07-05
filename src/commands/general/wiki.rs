use serenity::framework::standard::{macros::command, CommandResult};
use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::model::channel::Message;


// refazer usando recursos mais inteligentes
#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    let mut arg = &msg.content;

    if arg.contains("!f wiki") {
        arg.replace("!f wiki", "");
    } else if arg.contains("!fwiki") {  
        arg.replace("!fwiki", "");
    }

    let wiki_url = format!("https://pt.wikipedia.org/wiki/{}", arg.replace(" ", "_"));
    Ok(())
}