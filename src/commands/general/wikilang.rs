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
            match answer.content.to_lowercase().as_str() {
                "en" => {
                    match entry {
                        Ok(_) => {
                            sqlx::query!("DELETE FROM serverlang WHERE servid = ?;
                            INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                            guild_id, guild_id, languages::ENGLISH_LANGUAGE)
                            .execute(&database)
                            .await
                            .unwrap();
    
                            msg.reply_ping(ctx, "Now, the language is 🇺🇸|English!").await?;
                        },
                        Err(_) => {
                            sqlx::query!("INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                            guild_id, languages::ENGLISH_LANGUAGE)
                            .execute(&database)
                            .await
                            .unwrap();

                            msg.reply_ping(ctx, "Now, the language is 🇺🇸|English!").await?;
                        }
                    }
                },
                "pt" => {
                    match entry {
                        Ok(_) => {
                            sqlx::query!("DELETE FROM serverlang WHERE servid = ?;
                            INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                            guild_id, guild_id, languages::PORTUGUESE_LANGUAGE)
                            .execute(&database)
                            .await
                            .unwrap();

                            msg.reply_ping(ctx, "Now, the language is 🇧🇷|Portuguese!").await?;
                        },
                        Err(_) => {
                            sqlx::query!("INSERT INTO serverlang (servid, lang) VALUES (?, ?)",
                            guild_id, languages::PORTUGUESE_LANGUAGE)
                            .execute(&database)
                            .await
                            .unwrap();

                            msg.reply_ping(ctx, "Now, the language is 🇧🇷|Portuguese!").await?;
                        }
                    }
                },
                _ => {
                    
                }
            }
        } else {
            println!("excedeu 60 segs");
        }

    Ok(())
}
