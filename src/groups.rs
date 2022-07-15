use crate::commands::general::{wiki::WIKI_COMMAND, langlist::LANGLIST_COMMAND, wikilang::WIKILANG_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[commands(wiki, langlist, wikilang)]
struct General;