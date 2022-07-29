use super::*;

#[command]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn lang(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::lang();
    let guild_id = msg.guild_id.unwrap().0 as f64;

    let matches = command.try_get_matches_from(msg.content.to_clap_command(
        "!f".to_string().clone()))?;

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

    let new_language = matches.value_of("language").unwrap();

    if new_language == serv_lang {
        msg.reply_ping(&ctx.http, "❕ | That is already the current language!").await?;

    } else {
        let mut there_is: bool = false;
        for lang in ["en", "pt", "es", "de", "it", "fr", "ru"] {
            if new_language == lang {
                there_is = true;
                break;
            } else {
                there_is = false
            };
        }
    
        if there_is {
            let language_str = match new_language {
                "en" => {
                    "The current language is 🇺🇸 | English!"
                },
                "pt" => {
                    "The current language is 🇧🇷 | Portuguese!"
                },
                "es" => {
                    "The current language is 🇪🇸 | Spanish!"
                },
                "de" => {
                    "The current language is 🇩🇪 | Deutsch"
                },
                "it" => {
                    "The current language is 🇮🇹 | Italian"
                },
                "fr" => {
                    "The current language is 🇫🇷 | French"
                },
                "ru" => {
                    "The current language is 🇷🇺 | Russian"
                },
                "tr" => {
                    "The current language is 🇹🇷 | Turkish"
                }
                _ => {
                    ""
                },
            };

            match try_serv_lang {
                Ok(_) => {
                    sqlx::query!("DELETE FROM serverlang WHERE servid = ?;
                    INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                    guild_id, guild_id, new_language)
                    .execute(&database)
                    .await
                    .unwrap();

                    msg.reply_ping(&ctx.http, language_str).await?;
                },
                Err(_) => {
                    sqlx::query!("INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                    guild_id, new_language)
                    .execute(&database)
                    .await
                    .unwrap();

                    msg.reply_ping(&ctx.http, language_str).await?;
                }
            }
        } else {
            msg.reply_ping(&ctx.http, "❔ | No language with that abbreviation found.").await?;
        }
    }
    
    Ok(())
}