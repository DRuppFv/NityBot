use std::future::Future;
use std::sync::Arc;

use serenity::framework::standard::CommandResult;
use serenity::http::Http;
use serenity::model::channel::Message;

pub async fn handle_result(message: &Message, http: &Arc<Http>, res: impl Future<Output = CommandResult>) -> CommandResult {
    match res.await {
        Ok(_) => Ok(()),
        Err(why) => {
            message
                .reply_ping(http, "algo deu errado")
                .await?;
                println!("{}", why);
            Err(why)
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