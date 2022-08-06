use super::*;

#[command]
#[only_in(guilds)]
async fn randomlang(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, '⏳').await?;

    let command = commands::randomlang();

    let matches = command.try_get_matches_from(msg.content.to_clap_command(
    "!f".to_string().clone()))?;

    let language = matches.value_of("language").unwrap();

    let mut there_is: bool = false;
    for lang in ["en", "pt", "es", "de", "it", "fr", "ru", "tr", "nl"] {
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
        ctx, format!("https://{}.wikipedia.org/wiki/{}\n react with {} to delete this answer.", language,
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
