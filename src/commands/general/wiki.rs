use serenity::framework::standard::{macros::command, CommandResult};

#[command]
async fn wiki() -> CommandResult {
    Ok(())
}