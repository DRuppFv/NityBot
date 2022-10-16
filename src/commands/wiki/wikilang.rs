use super::*;

#[command]
#[only_in(guilds)]
async fn wikilang(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, '⏳').await?;

    let command = commands::wikilang();

    let mut string_lang: String = String::new();
    for (argnum, arg) in msg
        .content
        .replace("!f".clone(), "")
        .trim()
        .split(' ')
        .map(ToString::to_string)
        .enumerate()
    {
        if argnum == 1 {
            string_lang = format!("{}", arg)
        }
    }

    let mut vec_wiki: Vec<String> = Vec::new();
    let mut string_wiki: String = String::new();
    for (argnum, arg) in msg
        .content
        .replace("!f".clone(), "")
        .trim()
        .split(' ')
        .map(ToString::to_string)
        .enumerate()
    {
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
    for lang in ["en", "pt", "es", "de", "it", "fr", "ru", "tr", "nl"] {
        if language == lang {
            there_is = true;
            break;
        } else {
            there_is = false
        };
    }
    if !there_is {
        msg.reply_ping(&ctx.http, "❔ | No language with that abbreviation found.")
            .await?;
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
        categories_results: String::from("min"),
    };

    if &wiki_client.search(&wiki).unwrap().len() == &0 {
        msg.react(&ctx.http, '❔').await?;
        msg.delete_reaction_emoji(&ctx.http, '⏳').await?;
        return Ok(());
    }

    let wiki_page = wikipedia::Wikipedia::page_from_title(
        &wiki_client,
        (&wiki_client.search(&wiki).unwrap()[0]).to_string(),
    );

    let page_content = wiki_page.get_content().unwrap();
    let spaces = if page_content.matches(" ").count() < 76 {
        page_content.matches(" ").count()
    } else {
        76
    };
    let mut formatted_content = split_at_char(page_content.as_ref(), ' ', spaces - 1)
        .await
        .unwrap()
        .0;

    for char in [' ', '.', ',', ':', ';', '-', '+', '=', '(', ')'] {
        if formatted_content.chars().last().unwrap() == char {
            formatted_content = &formatted_content[..formatted_content.len() - 1]
        }
    }

    let mut sections = String::from("");
    if wiki_page.get_sections().unwrap().len() >= 5 {
        let mut formatted_section = String::from("");
        for section in &wiki_page.get_sections().unwrap()[0..5] {
            if section.len() > 15 {
                for char in [" ", "-", "&", "=", "_", "(", ")", "!", "[", "]"] {
                    if &section[14..15].to_string() == char {
                        formatted_section = format!("{}", section[..14].to_string());
                        break;
                    } else {
                        formatted_section = format!("{}", section[..15].to_string());
                    }
                }
                sections = format!("{}\n{}...", sections, formatted_section);
            } else {
                sections = format!("{}\n{}", sections, section);
            }
        }
    } else {
        let mut formatted_section = String::from("");
        for section in &wiki_page.get_sections().unwrap() {
            if section.len() > 15 {
                for char in [" ", "-", "&", "=", "_", "(", ")", "!", "[", "]"] {
                    if &section[14..15].to_string() == char {
                        formatted_section = format!("{}", section[..14].to_string());
                        break;
                    } else {
                        formatted_section = format!("{}", section[..15].to_string());
                    }
                }
                sections = format!("{}\n{}...", sections, formatted_section);
            } else {
                sections = format!("{}\n{}", sections, section);
            }
        }
    }

    let mut coordinates = String::new();
    let get_coordinates = wiki_page.get_coordinates().unwrap();
    if let Some(_) = get_coordinates {
        coordinates = format!(
            "Coordinates: {}°N | {}°E",
            get_coordinates.unwrap().0,
            get_coordinates.unwrap().1
        );
    }

    let wiki_answer = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!(
                "https://{}.wikipedia.org/wiki/{}",
                language,
                &wiki_client.search(&wiki).unwrap()[0].replace(" ", "_")
            ))
            .embed(|e| {
                e.title(format!("{}", &wiki_page.get_title().unwrap()))
                    .description(format!(
                        "{}\n{}...",
                        coordinates,
                        if &formatted_content.replace("\n\n", "").matches("owo").count()%2 == 0 {
                            formatted_content.replace("\n\n", "").replace("==", "**")
                        } else {
                            formatted_content.replace("\n\n", "")
                        }
                    ))
                    .fields(vec![("Few sections:", sections, true)])
                    .footer(|f| f.text(format!("React with {} to delete this answer.", '❌')))
                    .colour(Colour::from_rgb(91, 8, 199))
            })
        })
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
