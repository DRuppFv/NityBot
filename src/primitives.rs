use std::collections::BTreeMap;
use std::error::Error;

use rustbreak::backend::FileBackend;
use rustbreak::deser::Yaml;
use rustbreak::Database;

use serenity::prelude::TypeMapKey;

pub type ErrorBox<T> = Result<T, Box<dyn Error>>;

pub struct Prefixes;

pub trait ToCodeBlock {
    fn to_code_block(&self, t: &str) -> String;
}

pub trait ToClapCommand {
    fn to_clap_command(&self, prefix: String) -> Vec<String>;
}

pub const DEFAULT_PREFIX: &str = "sudo";

impl ToCodeBlock for str {
    fn to_code_block(&self, t: &str) -> String {
        format!("```{t}\n{self}```")
    }
}

impl ToCodeBlock for String {
    fn to_code_block(&self, t: &str) -> String {
        format!("```{t}\n{self}```")
    }
}

impl ToClapCommand for String {
    fn to_clap_command(&self, prefix: String) -> Vec<String> {
        self.replace(&prefix, "")
            .trim()
            .split(' ')
            .map(ToString::to_string)
            .collect()
    }
}

impl TypeMapKey for Prefixes {
    type Value = Database<BTreeMap<u64, String>, FileBackend, Yaml>;
}

pub mod commands {
    use clap::{Arg, Command};

    pub fn wiki() -> Command<'static> {
        Command::new("NAME: Wiki")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Sends the wiki of some specific thing.")
                .args([
                    Arg::new("wiki")
                        .required(true)
                        .index(1)
                        .help("The specific thing.")
                        ])
    }
}