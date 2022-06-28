use serenity::framework::standard::{macros::command, CommandResult};

#[command]
async fn commandone() -> CommandResult {
    Ok(())
}