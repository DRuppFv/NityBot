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

impl ToCodeBlock for String {
    fn to_code_block(&self, t: &str) -> String {
        format!("```{t}\n{self}```")
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
            .about("\nABOUT: Sends the wiki of some specific thing.")
                .args([
                    Arg::new("wiki")
                        .required(true)
                        .index(1)
                        .help("The specific thing.")
                ])
    }

    pub fn wikihelp() -> Command<'static> {
        Command::new("NAME: wikihelp")
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
            .about("\nABOUT: Shows a list of NityBot commands")
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
                        .help("The language to be set."),
                ])
    }

    pub fn langlist() -> Command<'static> {
        Command::new("NAME: langlist")
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Shows a list of the avaible languages.")
    }
}

