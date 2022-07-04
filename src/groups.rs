use crate::commands::general::{wiki::WIKI_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[commands(wiki)]
struct General;