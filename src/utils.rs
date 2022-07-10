use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandError},
    model::channel::Message,
};

use crate::primitives::ToCodeBlock;

#[hook]
pub async fn handle_result(
    ctx: &Context,
    message: &Message,
    _: &str,
    result: Result<(), CommandError>,
) {
    if let Err(why) = result {
        message.reply_ping(&ctx.http, why.to_string().to_code_block("yml"))
        .await
        .ok();
    }
}

#[macro_export]
macro_rules! exit {
    ($exit_code:expr, $($args:tt)*) => {{
        ::std::eprintln!($($args)*);
        ::std::process::exit($exit_code);
    }};
}