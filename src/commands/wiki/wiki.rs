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

    let mut vec_wiki: Vec<String> = Vec::new();
    let mut string_wiki: String = String::new();
    for (argnum, arg) in msg.content.replace("!f".clone(), "").trim().split(' ').map(ToString::to_string).enumerate() {
        if argnum != 0 {
            if argnum == 1 {
                string_wiki = format!("{}", arg)
            } else {
                string_wiki = format!("{} {}", string_wiki, arg)
            }
        }
    }
    vec_wiki.push("wiki".to_string());
    if string_wiki != "" {
        vec_wiki.push(string_wiki);
    }

    let matches_wiki = command.try_get_matches_from(&vec_wiki)?;

    let wiki = matches_wiki.value_of("wiki").unwrap();

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

    if &wiki_client.search(&wiki).unwrap().len() == &0 {
        msg.react(&ctx.http, '❔').await?;
        msg.delete_reaction_emoji(&ctx.http, '⏳').await?;
        return Ok(())
    }

    let wiki_page = wikipedia::Wikipedia::page_from_title(&wiki_client, (
        &wiki_client.search(&wiki).unwrap()[0]).to_string()).get_summary().unwrap();
    let oof = split_at_char(wiki_page.as_ref(), ' ', 75).await;

    let wiki_answer = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!("https://{}.wikipedia.org/wiki/{}", serv_lang,
        &wiki_client.search(&wiki).unwrap()[0].replace(" ", "_")))
        .embed(|e| {
            e.title(format!("{}", &wiki_client.search(&wiki).unwrap()[0]))
            .description(
                format!("{}...", oof.unwrap().0)
            )
            .fields(vec![
                ("f", "f", true),
            ])
            .footer(|f| f.text(format!("React with {} to delete this answer.", '❌')))
            .colour(Colour::from_rgb(91, 8, 199))           
        })
    })
    .await?;

    wiki_answer.react(&ctx.http, '❌').await?;
    msg.delete_reaction_emoji(&ctx.http, '⏳').await?;


    //let wiki_answer = msg.reply(
    //    ctx, format!("https://{}.wikipedia.org/wiki/{}\n react with {} to delete this answer.", serv_lang,
    //    &wiki_client.search(&wiki).unwrap()[0].replace(" ", "_"), '❌')
    //).await?;
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
