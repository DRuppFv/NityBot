use super::*;

#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, '⏳').await?;

    let command = commands::wiki();

    let guild_id = msg.guild_id.unwrap().0 as f64;

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
        search_results: 5,
        images_results: String::from("min"),
        links_results: String::from("min"),
        categories_results: String::from("min")
    };

    let mut vec: Vec<String> = Vec::new();
    let mut string: String = String::new();
    for (argnum, arg) in msg.content.replace("!f".clone(), "").trim().split(' ').map(ToString::to_string).enumerate() {
        if argnum != 0 {
            if argnum == 1 {
                string = format!("{}", arg)
            } else {
                string = format!("{} {}", string, arg)
            }
        } else {
            vec.push("wiki".to_string());
        };
    }
    if string != "" {
        vec.push(string);
    }

    let matches = command.try_get_matches_from(&vec)?;

    let wiki = matches.value_of("wiki").unwrap();

    if &wiki_client.search(&wiki).unwrap().len() == &0 {
        msg.react(&ctx.http, '❔').await?;
        msg.delete_reaction_emoji(&ctx.http, '⏳').await?;
        return Ok(())
    }

    msg.reply(
        ctx, format!("https://{}.wikipedia.org/wiki/{}", serv_lang,
        &wiki_client.search(&wiki).unwrap()[0].replace(" ", "_"))
    ).await?;
    msg.delete_reaction_emoji(&ctx.http, '⏳').await?;
    
    Ok(())
}
