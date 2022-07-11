pub mod general;

use std::time::Duration;

pub use wikipedia::Wikipedia;

pub use crate::{
    primitives::{DEFAULT_LANGUAGE, DEFAULT_LANGUAGE_NAME, commands, ToClapCommand},
    utils::handle_result,
};

pub use serenity::{
    async_trait,
    utils::Colour,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{ Timestamp, prelude::MessageId, channel::Message, id::UserId},
};
