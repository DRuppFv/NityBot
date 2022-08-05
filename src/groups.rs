use crate::commands::{
    general::{
        help::HELP_COMMAND,
        lang::LANG_COMMAND,
        langlist::LANGLIST_COMMAND,
    },
    random::{
        random::RANDOM_COMMAND,
        randomlang::RANDOMLANG_COMMAND,
    },
    wiki::{
        wiki::WIKI_COMMAND,
        wikilang::WIKILANG_COMMAND,
    },
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