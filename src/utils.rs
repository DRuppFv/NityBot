use clap::ErrorKind;
use clap::Error;

use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandError},
    model::channel::Message,
};

#[hook]
pub async fn handle_result(
    ctx: &Context,
    message: &Message,
    _: &str,
    result: Result<(), CommandError>,
) {
    if let Err(_) = result {
        match result.unwrap_err().downcast_ref::<Error>().unwrap().kind() {
            ErrorKind::MissingRequiredArgument => {
                message.reply(&ctx.http,"...").await.ok();
            },
            _ => {
                message.reply(&ctx.http,"Undefined error.").await.ok();
            }
        }
    }
}

#[macro_export]
macro_rules! exit {
    ($exit_code:expr, $($args:tt)*) => {{
        ::std::eprintln!($($args)*);
        ::std::process::exit($exit_code);
    }};
}