use super::*;

#[command]
#[only_in(guilds)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::help();

    command.try_get_matches_from(msg.content.to_clap_command("!f".to_string().clone()))?;

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title("🔸Help Command")
            .fields(vec![
                ("🔹Commands list:", 
                "🔹!flanglist\n  |Shows the current language and a list of the avaiables languages.
                \n🔹!flang [language]\n  |Sets a language to a server.
                \n🔹!fwiki [search]\n  |Sends the wiki of something specific in the current server/guild language.
                \n🔹!fwikilang [language] [search]\n  |Sends a specific wiki in the current server/guild language.
                \n🔹!frandom\n  |Sends a random wiki in the current server/guild language.
                \n", 
                true),
            ])
            .colour(Colour::from_rgb(91, 8, 199))           
        })
    })
    .await?;
    
    Ok(())
}