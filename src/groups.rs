use crate::commands::general::{commandone::COMMANDONE_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[commands(commandone)]
struct General;