use super::*;

#[command]
#[only_in(guilds)]
async fn wikilang(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, '⏳').await?;

    let command = commands::wikilang();

    let mut string_lang: String = String::new();
    for (argnum, arg) in msg.content.replace("!f".clone(), "").trim().split(' ').map(ToString::to_string).enumerate() {
        if argnum == 1 {
            string_lang = format!("{}", arg)
        }
    }

    let mut vec_wiki: Vec<String> = Vec::new();
    let mut string_wiki: String = String::new();
    for (argnum, arg) in msg.content.replace("!f".clone(), "").trim().split(' ').map(ToString::to_string).enumerate() {
        if argnum > 1 {
            if argnum == 2 {
                string_wiki = format!("{}", arg)
            } else {
                string_wiki = format!("{} {}", string_wiki, arg)
            }
        };
    }
    vec_wiki.push("wiki".to_string());
    vec_wiki.push(string_lang);
    if string_wiki != "" {
        vec_wiki.push(string_wiki);
    }
    
    let matches_lang = command.try_get_matches_from(&vec_wiki)?;

    let language = matches_lang.value_of("language").unwrap();
    let wiki = matches_lang.value_of("wiki").unwrap();

    let mut there_is: bool = false;
    for lang in ["en", "pt", "es", "de", "it", "fr", "ru", "tr"] {
        if language == lang {
            there_is = true;
            break;
        } else {
            there_is = false
        };
    }
    if !there_is {
        msg.reply_ping(&ctx.http, "❔ | No language with that abbreviation found.").await?;
        msg.delete_reaction_emoji(&ctx.http, '⏳').await?;
        return Ok(());
    }

    let wiki_client = wikipedia::Wikipedia {
        client: <wikipedia::http::default::Client>::default(),
        pre_language_url: String::from("https://"),
        post_language_url: String::from(".wikipedia.org/w/api.php"),
        language: String::from(language),
        search_results: 5,
        images_results: String::from("min"),
        links_results: String::from("min"),
        categories_results: String::from("min"),
    };

    if &wiki_client.search(&wiki).unwrap().len() == &0 {
        msg.react(&ctx.http, '❔').await?;
        msg.delete_reaction_emoji(&ctx.http, '⏳').await?;
        return Ok(());
    }

    let wiki_answer = msg
        .reply(
            ctx,
            format!(
                "https://{}.wikipedia.org/wiki/{}\n react with {} to delete this answer.",
                language,
                &wiki_client.search(&wiki).unwrap()[0].replace(" ", "_"),
                '❌'
            ),
        )
        .await?;
    wiki_answer.react(&ctx.http, '❌').await?;
    msg.delete_reaction_emoji(&ctx.http, '⏳').await?;

    loop {
        if let Some(reaction) = &wiki_answer
            .await_reaction(&ctx)
            .author_id(msg.author.id)
            .await
        {
            let emoji = &reaction.as_inner_ref().emoji;

            let _ = match emoji.as_data().as_str() {
                "❌" => {
                    wiki_answer.delete(&ctx.http).await?;
                    break;
                }
                _ => {}
            };
        }
    }

    Ok(())
}
