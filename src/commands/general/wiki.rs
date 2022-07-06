use super::*;

#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(msg, &ctx.http, async move {
        let command = commands::wiki();
        let guild_id = *msg.guild_id.unwrap().as_u64();
        let prefix = get_prefix(ctx.data.clone(), guild_id).await;

        let matches = command.try_get_matches_from(msg.content.to_clap_command(prefix))?;

        Ok(())
    })
    .await?;

    Ok(())
}