use super::*;

#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(msg, &ctx.http, async move {
        let command = commands::wiki();
        let arg = format!("{}{}", &msg.content[..8], &msg.content[8..].replace(" ", "_"));
        println!("{}", arg);
        let matches = command.try_get_matches_from(
            arg.to_clap_command("!f".to_string().clone()))?;

        let wiki = matches.value_of("wiki_subject").unwrap();

        msg.reply(ctx, format!("https://pt.wikipedia.org/wiki/{}", wiki)).await?;

        Ok(())
    })
    .await
}
