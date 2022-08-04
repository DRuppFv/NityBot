pub const DEFAULT_LANGUAGE: &str = "en";

pub trait ToClapCommand {
    fn to_clap_command(&self, prefix: String) -> Vec<String>;
}

pub trait ToCodeBlock {
    fn to_code_block(&self, t: &str) -> String;
}

impl ToClapCommand for String {
    fn to_clap_command(&self, prefix: String) -> Vec<String> {
        self.replacen(&prefix, "", 1)
            .trim()
            .split(' ')
            .map(ToString::to_string)
            .collect()
    }
}

pub mod commands {
    use clap::{Arg, Command};

    pub fn wiki() -> Command<'static> {
        Command::new("NAME: wiki")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Sends the wiki of something specific in the current server/guild language.")
            .args([
                Arg::new("wiki")
                    .required(true)
                    .index(1)
                    .help("The specific thing.")
            ])
    }

    pub fn help() -> Command<'static> {
        Command::new("NAME: help")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Shows a list of NityBot commands")
    }

    pub fn wikilang() -> Command<'static> {
        Command::new("NAME: wikilang")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Sends a specific wiki in the selected language.")
            .args([
                Arg::new("language")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("The language which the wikiclient will use."),
                Arg::new("wiki")
                    .required(true)
                    .index(2)
                    .help("The specific thing.")
            ])
    }

    pub fn random() -> Command<'static> {
        Command::new("NAME: random")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Sends a random wiki in the current server/guild language")
    }

    pub fn randomlang() -> Command<'static> {
        Command::new("NAME: randomlang")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Sends a random wiki in the selected language.")
            .args([
                Arg::new("language")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("The language that will be used."),
            ])
    }

    pub fn lang() -> Command<'static> {
        Command::new("NAME: lang")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Sets a language to a server.")
            .args([
                Arg::new("language")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("The language which the wikiclient will use."),
            ])
    }

    pub fn langlist() -> Command<'static> {
        Command::new("NAME: langlist")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Shows the current language and a list of the avaiables languages.")
    }
}

