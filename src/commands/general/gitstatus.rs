use serenity::framework::standard::{macros::command, CommandResult};

#[command]
async fn gitstatus() -> CommandResult {
    Ok(())
}