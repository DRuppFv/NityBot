use crate::commands::general::{
    help::HELP_COMMAND,
    lang::LANG_COMMAND,
    langlist::LANGLIST_COMMAND,
};

use crate::commands::random::{
    randomlang::RANDOMLANG_COMMAND,
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
#[commands(random, randomlang)]
struct Random;

#[group]
#[commands(wiki, wikilang)]
struct Wiki;