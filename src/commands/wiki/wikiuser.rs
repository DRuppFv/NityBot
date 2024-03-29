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

    let member = msg.guild_id.unwrap().member(&ctx.http, &user.id).await?;

    let mut fields = vec![
        ("Joined discord:", format!("<t:{}:R>", user.created_at().unix_timestamp()), true),   //fields
        ("Joined server:", format!("<t:{}:R>", member.joined_at.unwrap().unix_timestamp()), true),
    ];

    if let Some(x) = member.premium_since {
        fields.push(("Boosting since:", format!("<t:{}:R>", x.unix_timestamp()), true))
    }

    let user_nick_in = user.nick_in(&ctx.http, msg.guild_id.unwrap()).await
        .unwrap_or(user.name.to_string());
        
    let user_avatar = &user.face();

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a: &mut serenity::builder::CreateEmbedAuthor|
                a.icon_url(member.face())
                .name(user_nick_in))
                .thumbnail(member.face())
                .fields(fields)
                .footer(|f| 
                f.icon_url(user_avatar)
                .text(user.tag()))
                .colour(Colour::from_rgb(91, 8, 199))
            })
        })
        .await?;

    Ok(())
}
