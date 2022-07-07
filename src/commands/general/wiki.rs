use super::*;

#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(msg, &ctx.http, async move {
        let command = commands::wiki();

        let matches = command.try_get_matches_from(
        msg.content.to_clap_command("!f".to_string().clone(), "wiki"))?;

        let wiki = matches.value_of("wiki_subject").unwrap();
        println!("{}", wiki);
        
        msg.reply(ctx, format!("https://pt.wikipedia.org/wiki/{}", wiki.replace(" ", ""))).await?;

        Ok(())
    })
    .await
}
