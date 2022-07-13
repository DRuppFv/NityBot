use super::*;

#[command]
async fn wikilang(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::wikilang();

    let guild_id = msg.guild_id.unwrap().0 as f64;
    command.try_get_matches_from(msg.content.to_clap_command("!fv".to_string()))?;

    let database = sqlx::sqlite::SqlitePoolOptions::new().max_connections(5)
    .connect_with(
        sqlx::sqlite::SqliteConnectOptions::new().filename("languages.db"),
    ).await.expect("");

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

    let entry = sqlx::query!(
        "SELECT rowid FROM serverlang WHERE servid = ?",
        guild_id,
    )
    .fetch_one(&database)
    .await;

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
                    "Now, the language is 🇺🇸|English!"
                },
                "pt" => {
                    "Now, the language is 🇧🇷|Portuguese!"
                },
                _ => {
                    ""
                }
            };

            match entry {
                Ok(_) => {
                    sqlx::query!("DELETE FROM serverlang WHERE servid = ?;
                    INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                    guild_id, guild_id, answer_str)
                    .execute(&database)
                    .await
                    .unwrap();

                    msg.reply_ping(ctx, language_str).await?;
                },
                Err(_) => {
                    sqlx::query!("INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                    guild_id, answer_str)
                    .execute(&database)
                    .await
                    .unwrap();

                    msg.reply_ping(ctx, language_str).await?;
                }
            }
        } else {
            msg.reply_ping(ctx, "❔ | No language with that abbreviation found.").await?;
        }
    } else {
        msg.reply_ping(ctx, "🕐 | You waited too long. Call the command again!").await?;
    }
    
    Ok(())
}
