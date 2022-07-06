pub mod general;

pub use std::time::{Duration, Instant};

pub use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::UserId, prelude::MessageId},
};

pub use tokio::time::sleep;

pub use crate::{
    primitives::{commands, ToClapCommand},
    utils::{handle_result}
};