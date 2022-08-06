use super::*;

#[command]
#[only_in(guilds)]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, '⏳').await?;

    let command = commands::random();
    let guild_id = msg.guild_id.unwrap().0 as f64;

    command.try_get_matches_from(msg.content.to_clap_command("!f".to_string().clone()))?;

    let database = sqlx::sqlite::SqlitePoolOptions::new().max_connections(5)
    .connect_with(
        sqlx::sqlite::SqliteConnectOptions::new().filename("languages.db"),
    ).await.expect("");

    let try_serv_lang = &sqlx::query!(
        "SELECT lang FROM serverlang WHERE servid = ?",
        guild_id,
    )
    .fetch_one(&database)
    .await;

    let serv_lang: &str = if let Ok(x) = try_serv_lang {
        &x.lang
    } else {
        DEFAULT_LANGUAGE
    };

    let wiki_client = wikipedia::Wikipedia {
        client: <wikipedia::http::default::Client>::default(),
        pre_language_url: String::from("https://"),
        post_language_url: String::from(".wikipedia.org/w/api.php"),
        language: String::from(serv_lang),
        search_results: 1,
        images_results: String::from("min"),
        links_results: String::from("min"),
        categories_results: String::from("min")
    };

    let wiki = &wiki_client.random().unwrap().unwrap();

    if &wiki_client.search(&wiki).unwrap().len() == &0 {
        msg.react(&ctx.http, '❔').await?;
        msg.delete_reaction_emoji(&ctx.http, '⏳').await?;
        return Ok(())
    }

    let wiki_answer = msg.reply(
        ctx, format!("https://{}.wikipedia.org/wiki/{}\n react with {} to delete this answer.", serv_lang,
        &wiki_client.search(&wiki).unwrap()[0].replace(" ", "_"), '❌')
    ).await?;
    wiki_answer.react(&ctx.http, '❌').await?;
    msg.delete_reaction_emoji(&ctx.http, '⏳').await?;

    loop {
        if let Some(reaction) = &wiki_answer
        .await_reaction(&ctx)
        .author_id(msg.author.id)
        .await {
            let emoji = &reaction.as_inner_ref().emoji;

            let _ = match emoji.as_data().as_str() {
                "❌" => {
                    wiki_answer.delete(&ctx.http).await?;
                    break
                },
                _ => {
                    
                }
            };
        }
    }

    Ok(()) 
}