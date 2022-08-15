pub mod general;
pub mod random;
pub mod wiki;

pub use wikipedia::Wikipedia;

pub use crate::{
    primitives::{commands, ToClapCommand, DEFAULT_LANGUAGE},
    utils::{handle_result, split_at_char},
};

pub use serenity::{
    async_trait,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::UserId, prelude::MessageId, Timestamp},
    utils::Colour,
};
