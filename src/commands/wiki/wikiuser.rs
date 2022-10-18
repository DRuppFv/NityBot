use super::*;

#[command]
#[only_in(guilds)]
async fn wikiuser(ctx: &Context, msg: &Message) -> CommandResult {
    msg.react(&ctx.http, '⏳').await?;

    let command = commands::wikiuser();

    let matches = command.try_get_matches_from(msg.content.to_clap_command("!f".to_string().clone()))?;

    let mention = matches.value_of("mention").unwrap().parse::<Mention>();
    if let Err(_) = mention {
        msg
        .reply(&ctx.http, "❕ | Invalid mention!")
        .await?;

        return Ok(());
    }
    
    let _user = if let serenity::model::mention::Mention::User(UserId(id)) = mention.unwrap() {
        UserId::from(id).to_user(&ctx.http).await?
    } else {
        msg
        .reply(&ctx.http, "❕ | Be sure that you mentioned a user!")
        .await?;

        return Ok(());
    };

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!(
                "content"
            ))
            .embed(|e| {
                e.title(format!("title"))
                    .description(format!(
                        "desc"
                    ))
                    .fields(vec![("fields", "field 1", true)])
                    .footer(|f| f.text(format!("")))
                    .colour(Colour::from_rgb(91, 8, 199))
            })
        })
        .await?;

    Ok(())
}
