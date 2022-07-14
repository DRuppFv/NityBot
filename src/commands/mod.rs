pub mod general;
pub mod slash;

use std::time::Duration;

pub use wikipedia::Wikipedia;

pub use crate::{
    primitives::{commands, ToClapCommand, DEFAULT_LANGUAGE},
    utils::handle_result,
};

pub use serenity::{
    async_trait,
    utils::Colour,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{ Timestamp, prelude::MessageId, channel::Message, id::UserId},
};
