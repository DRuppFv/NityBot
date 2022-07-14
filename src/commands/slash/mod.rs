mod wikilang;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        interactions::{
            application_command::ApplicationCommandInteraction,
            message_component::MessageComponentInteraction, Interaction,
        },
    },
};
use wikilang::Wikilang;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => match command.data.name.as_str() {
                "wikilang" => {
                    Wikilang.handle_call(&ctx, &command).await;
                }
                _ => {}
            },
            Interaction::MessageComponent(component) => {
                match component.data.custom_id.as_str() {
                    //used by Wikilang's buttons
                    "333" | "334" => {
                        Wikilang.handle_message_component(&ctx, &component).await;
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online", ready.user.name);
        Wikilang.create_command(&ctx).await;
    }
}

#[async_trait]
pub trait SlashCommand: Send + Sync {
    async fn create_command(&self, ctx: &Context);
    async fn handle_call(&self, ctx: &Context, interaction: &ApplicationCommandInteraction);
    async fn handle_message_component(
        &self,
        _ctx: &Context,
        _interaction: &MessageComponentInteraction,
    ) {
    }
}
