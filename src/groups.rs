use crate::commands::general::{
    lang::LANG_COMMAND,
    langlist::LANGLIST_COMMAND,
};

use crate::commands::wiki::{
    wiki::WIKI_COMMAND,
    wikihelp::WIKIHELP_COMMAND,
    wikilang::WIKILANG_COMMAND,
};

use serenity::framework::standard::macros::group;

#[group]
#[commands(lang, langlist)]
struct General;

#[group]
#[commands(wiki, wikihelp, wikilang)]
struct Wiki;