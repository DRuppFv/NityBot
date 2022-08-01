use crate::commands::general::{
    help::HELP_COMMAND,
    lang::LANG_COMMAND,
    langlist::LANGLIST_COMMAND,
};

use crate::commands::random::{
    random::RANDOM_COMMAND,
};

use crate::commands::wiki::{
    wiki::WIKI_COMMAND,
    wikilang::WIKILANG_COMMAND,
};

use serenity::framework::standard::macros::group;

#[group]
#[commands(help, lang, langlist)]
struct General;

#[group]
#[commands(random)]
struct Random;

#[group]
#[commands(wiki, wikilang)]
struct Wiki;