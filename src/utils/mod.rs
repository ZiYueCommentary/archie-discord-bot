use poise::{CreateReply, ReplyHandle};
use serenity::Error;
use tracing::log::warn;
use crate::{i18n, Context};

pub trait ReplyHandleExt {
    async fn append(&self, ctx: Context<'_>, content: &str) -> Result<(), Error>;
}

pub trait ContextExt {
    async fn reply_locale(&self, key: &'static str) -> Result<ReplyHandle<'_>, Error>;
}

impl ReplyHandleExt for ReplyHandle<'_> {
    async fn append(&self, ctx: Context<'_>, content: &str) -> Result<(), Error> {
        let previous = match self.to_owned().into_message().await {
            Ok(message) => message.content,
            Err(e) => {
                warn!("Can't fetch previous message: {e}");

                String::new()
            }
        };
        self.edit(ctx, CreateReply::default().content(format!("{previous}\n{content}"))).await
    }
}

impl ContextExt for Context<'_> {
    async fn reply_locale(&self, key: &'static str) -> Result<ReplyHandle<'_>, Error> {
        self.reply(i18n::get(self.locale().unwrap(), key)).await
    }
}