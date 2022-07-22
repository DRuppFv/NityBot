use super::*;

#[command]
#[only_in(guilds)]
async fn wikihelp(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::wikihelp();

    command.try_get_matches_from(msg.content.to_clap_command("!f".to_string().clone()))?;

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("🔸Help Command")
            .fields(vec![
                ("🔹Commands list:", 
                "🔹!flanglist\n  |Shows the current language and a list of the avaiables languages.\n\n🔹!flang [language]\n  |Sets a language to a server.\n\n🔹!fwiki [search]\n  |Searches for [search] in wikipedia.\n\n🔹!fwiki [language] [search]\n  |Searches for [search] with [language] in wikipedia.\n\n", true),
            ])
            .colour(Colour::from_rgb(91, 8, 199))           
        })
    })
    .await?;
    
    Ok(())
}