use crate::commands::general::{gitstatus::GITSTATUS_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[commands(gitstatus)]
struct General;