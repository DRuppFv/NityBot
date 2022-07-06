use std::future::Future;
use std::sync::Arc;

use serenity::framework::standard::CommandResult;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::prelude::{RwLock, TypeMap};

use crate::primitives::{Prefixes, ToCodeBlock, DEFAULT_PREFIX};

pub async fn get_prefix(data: Arc<RwLock<TypeMap>>, guild_id: u64) -> String {
    data.read()
        .await
        .get::<Prefixes>()
        .unwrap()
        .get_data(true)
        .unwrap()
        .get(&guild_id)
        .unwrap_or(&DEFAULT_PREFIX.into())
        .clone()
}

pub async fn handle_result(message: &Message, http: &Arc<Http>, res: impl Future<Output = CommandResult>) -> CommandResult {
    match res.await {
        Ok(_) => Ok(()),
        Err(why) => {
            message
                .reply_ping(http, why.to_string().to_code_block("yml"))
                .await?;
            Err(why)
        }
    }
}