use crate::utils::ReplyHandleExt;
use crate::{Context, Error, i18n};
use poise::ReplyHandle;
use std::time::Duration;
use tokio::time::sleep;

/// sudo pacman -Syu
#[poise::command(slash_command)]
pub async fn pacman(ctx: Context<'_>) -> Result<(), Error> {
    let reply: ReplyHandle = ctx
        .reply(format!(
            ":: {}",
            i18n::get(ctx.locale().unwrap(), "pacman.sync_packages")
        ))
        .await?;
    sleep(Duration::from_secs(1)).await;
    reply
        .append(ctx, &format!(":: {}", i18n::get(ctx.locale().unwrap(), "pacman.full_upgrade")))
        .await?;
    sleep(Duration::from_secs(1)).await;
    // :: Replace ... with ...? [Y/n]
    reply.append(ctx, "resolving dependencies...").await?;
    sleep(Duration::from_secs(1)).await;
    reply
        .append(ctx, "looking for conflicting packages...")
        .await?;
    Ok(())
}
