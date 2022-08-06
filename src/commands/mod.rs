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
    utils::Colour,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{ Timestamp, prelude::MessageId, channel::Message, id::UserId},
};
