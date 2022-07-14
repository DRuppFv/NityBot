use serenity::{
    async_trait,
    client::Context,
    model::interactions::{
        application_command::ApplicationCommand, message_component::MessageComponentInteraction,
        InteractionResponseType,
    },
};

use crate::api::wikilang::{Guild, Lang};

use super::SlashCommand;

pub struct Wikilang;

#[async_trait]
impl SlashCommand for Wikilang {
    async fn create_command(&self, ctx: &serenity::client::Context) {
        ApplicationCommand::create_global_application_command(&ctx.http, |command| {
            command
                .name("wikilang")
                .description("set wikipedia language")
        })
        .await
        .unwrap();
    }
    async fn handle_call(
        &self,
        ctx: &Context,
        interaction: &serenity::model::interactions::application_command::ApplicationCommandInteraction,
    ) {
        interaction
            .create_interaction_response(&ctx.http, |f| {
                f.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.embed(|embed| {
                            embed
                                .title("Choose the language")
                                .description("Click the button to choose the language")
                        })
                        .components(|f| {
                            f.create_action_row(|row| {
                                row.create_button(|button| {
                                    button.custom_id("333").label("Portuguese 🇧🇷")

                                })
                                .create_button(|button| button.custom_id("334").label("English 🇺🇸‎"))
                            })
                        })
                    })
            })
            .await
            .ok();
    }
    async fn handle_message_component(
        &self,
        ctx: &Context,
        interaction: &MessageComponentInteraction,
    ) {
        let lang = match interaction.data.custom_id.as_str() {
            "333" => {
                Lang::Pt
            },
            "334" => {
                Lang::En
            },
            _ => {
                Lang::En
            }
        };
        let guild = interaction.guild_id.unwrap().0 as i64;
        //TODO: rewrite this
        let guild = if let Some(mut guild) = Guild::from_db(guild).await {
            guild.set_lang(lang).await.ok();
            guild
        }else {
            let guild = Guild::new(guild, lang).await;
            guild.save().await.ok();
            guild
        };
        interaction.create_interaction_response(&ctx.http, |f| {
            f
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|resp| {
                    resp.content(format!("the language of is now {:?}!", guild.lang))
                })
        }).await.unwrap();
    }
}
