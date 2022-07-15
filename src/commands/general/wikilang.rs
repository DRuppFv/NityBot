use super::*;

#[command]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn wikilang(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::wikilang();
    let guild_id = msg.guild_id.unwrap().0 as f64;

    let matches = command.try_get_matches_from(msg.content.to_clap_command("!f".to_string().clone()))?;

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

    if matches.is_present("show") {
        if matches.value_of("show").unwrap() == "show" {
            msg.reply_ping(&ctx.http, format!("{}", 
            match serv_lang {
                "en" => {
                    "The current language is 🇺🇸|English!"
                },
                "pt" => {
                    "The current language is 🇧🇷|Portuguese!"
                },
                _ => {
                    ""
                }
            })).await?;

            return Ok(());
      } else {
        msg.reply_ping(&ctx.http, format!("❔ | No argument '{}' found.", matches.value_of("show").unwrap())).await?;
        return Ok(());
      }
    }

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Choose your language!")
            .description("Reply this with your choice!")
            .fields(vec![
                ("\nLanguages:", "🇺🇸 | English - en\n🇧🇷 | Portuguese - pt\n", true),
            ])
            .timestamp(Timestamp::now())
            .colour(Colour::from_rgb(91, 8, 199))           
        })
    })
    .await?;

    if let Some(answer) = &msg.author.await_reply(&ctx)
    .timeout(Duration::from_secs(60)).await {
        let answer_str = answer.content.to_lowercase();

        let mut there_is: bool = false;
        for lang in ["en", "pt"] {
            if answer_str.as_str() == lang {
                there_is = true;
                break;
            } else {
                there_is = false
            };
        }
    
        if there_is {
            let language_str = match answer_str.as_str() {
                "en" => {
                    "The current language is 🇺🇸|English!"
                },
                "pt" => {
                    "The current language is 🇧🇷|Portuguese!"
                },
                _ => {
                    ""
                }
            };

            match try_serv_lang {
                Ok(_) => {
                    sqlx::query!("DELETE FROM serverlang WHERE servid = ?;
                    INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                    guild_id, guild_id, answer_str)
                    .execute(&database)
                    .await
                    .unwrap();

                    msg.reply_ping(&ctx.http, language_str).await?;
                },
                Err(_) => {
                    sqlx::query!("INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                    guild_id, answer_str)
                    .execute(&database)
                    .await
                    .unwrap();

                    msg.reply_ping(&ctx.http, language_str).await?;
                }
            }
        } else {
            msg.reply_ping(&ctx.http, "❔ | No language with that abbreviation found.").await?;
        }
    } else {
        msg.reply_ping(&ctx.http, "🕐 | You waited too long. Call the command again!").await?;
    }
    
    Ok(())
}
