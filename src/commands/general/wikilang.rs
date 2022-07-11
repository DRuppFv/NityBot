use super::*;

#[command]
async fn wikilang(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::wikilang();
    command.try_get_matches_from(msg.content.to_clap_command("!fv".to_string()))?;


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
            match answer.content.to_lowercase().as_str() {
                "pog" => {

                },
                _ => {
                    
                }
            }
        }

    Ok(())
}