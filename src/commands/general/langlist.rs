use super::*;

#[command]
#[only_in(guilds)]
async fn langlist(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::langlist();
    let guild_id = msg.guild_id.unwrap().0 as f64;

    command.try_get_matches_from(msg.content.to_clap_command("!f".to_string().clone()))?;

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

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("Current language:")
            .description(match serv_lang {
                "en" => {
                    "šŗšø | English"
                },
                "pt" => {
                    "š§š· | Portuguese"
                },
                "es" => {
                    "šŖšø | Spanish!"
                },
                "de" => {
                    "š©šŖ | Deutsch"  
                },
                "it" => {
                    "š®š¹ | Italian"
                },
                "fr" => {
                    "š«š· | French"
                },
                "ru" => {
                    "š·šŗ | Russian"
                },
                "tr" => {
                    "š¹š· | Turkish"
                },
                "nl" => {
                    "š³š± | Dutch"
                },
                _ => {
                    ""
                }
            })
            .fields(vec![
                ("Avaiable languages:", "šŗšø | English - en\nš§š· | Portuguese - pt\nšŖšø | Spanish - es
                š©šŖ | Deutsch - de\nš®š¹ | Italian - it\nš«š· | French - fr\nš·šŗ | Russian - ru\nš¹š· | Turkish - tr\nš³š± | Dutch - nl\n", true),
            ])
            .footer(|f| f.text("Choose your language with !flang [lang]."))
            .colour(Colour::from_rgb(91, 8, 199))           
        })
    })
    .await?;

    Ok(())
}