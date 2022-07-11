use crate::commands::general::{wiki::WIKI_COMMAND, wikilang::WIKILANG_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[commands(wiki, wikilang)]
struct General;