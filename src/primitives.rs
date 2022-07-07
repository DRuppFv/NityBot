pub trait ToClapCommand {
    fn to_clap_command(&self, prefix: String, split_char: &str) -> Vec<String>;
}

impl ToClapCommand for String {
    fn to_clap_command(&self, prefix: String, split_char: &str) -> Vec<String> {
        self.replace(&prefix, "")
            .trim()
            .split(split_char)
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
            .about("\nABOUT: Sends the wiki of some specific thing.")
                .args([
                    Arg::new("wiki_subject")
                        .required(true)
                        .index(1)
                        .help("The specific thing.")
                        ])
    }
}