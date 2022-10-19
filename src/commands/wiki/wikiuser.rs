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
    
    let user = if let serenity::model::mention::Mention::User(UserId(id)) = mention.unwrap() {
        UserId::from(id).to_user(&ctx.http).await?
    } else {
        msg
        .reply(&ctx.http, "❕ | Be sure that you mentioned a user!")
        .await?;

        return Ok(());
    };


    let user_nick_in = user.nick_in(&ctx.http, msg.guild_id.unwrap()).await
        .unwrap_or(user.name.to_string());
        
    let user_avatar = &user.avatar_url().unwrap();

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!(
                ""                       //content
            ))
            .embed(|e| {
                e.title(format!(""))                   //title
                .author(|a: &mut serenity::builder::CreateEmbedAuthor|
                a.icon_url(user_avatar)
                .name(user_nick_in))
                .thumbnail(user_avatar)
                .description(format!(
                    ""                      //description
                ))
                .fields(vec![
                    ("", "", true),   //fields
                    ("", "", true)
                    ])
                .footer(|f| 
                f.icon_url(user_avatar)
                .text(user.tag()))
                .colour(Colour::from_rgb(91, 8, 199))
            })
        })
        .await?;

    Ok(())
}
