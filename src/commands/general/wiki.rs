use super::*;

#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(msg, &ctx.http, async move {
        let command = commands::wiki();

        let matches = command.try_get_matches_from(
        msg.content.to_clap_command("!f".to_string().clone(), "wiki"))?;

        let wiki = matches.value_of("wiki_subject").unwrap().replacen(" ", "", 1);

        let wikipe = wikipedia::Wikipedia {
            client: <wikipedia::http::default::Client>::default(),
            pre_language_url: String::from("https://"),
            post_language_url: String::from(".wikipedia.org/w/api.php"),
            language: String::from("pt"),
            search_results: 3,
            images_results: String::from("min"),
            links_results: String::from("min"),
            categories_results: String::from("min")
        };

        let result = &wikipe.search(&wiki).unwrap()[0];

        msg.reply(
            ctx, format!("https://pt.wikipedia.org/wiki/{}", result.replace(" ", "_"))
        ).await?;

        Ok(())
    })
    .await
}
