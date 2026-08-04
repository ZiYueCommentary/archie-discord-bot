pub(crate) mod ask;
pub(crate) mod pacman;

use crate::{Context, Error, i18n, userdata};
use formatx::formatx;
use poise::CreateReply;
use serenity::all::{Colour, CreateAttachment, CreateEmbed};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tracing::log::error;

/// I'm femboy btw
#[poise::command(slash_command)]
pub async fn nyaofetch(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().attachment(
            CreateAttachment::file(
                &fs::File::open("assets/nyarch.png")
                    .await
                    .expect("Cannot find `nyarch.png`, is working directory broken?"),
                "nyarch.png",
            )
            .await
            .unwrap(),
        ),
    )
    .await?;
    Ok(())
}

/// I use Arch btw
#[poise::command(slash_command)]
pub async fn fastfetch(ctx: Context<'_>) -> Result<(), Error> {
    let channel_name = match ctx.guild_channel().await {
        Some(channel) => channel.name,
        None => "D1rectMe55age".to_string(),
    };
    let timestamp_secs = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(e) => {
            error!("Wrong system time! Is it earlier than UNIX EPOCH? {e:?}");

            0
        }
    };

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("fastfetch")
                .color(Colour::new(0x57bfbf))
                .description(format!(
                    "```
                  -`
                 .o+`
                `ooo/
               `+oooo:
              `+oooooo:
              -+oooooo+:
            `/:-:++oooo+:
           `/++++/+++++++:
          `/++++++++++++++:
         `/+++ooooooooooooo/`
        ./ooosssso++osssssso+`
       .oossssso-`` `/ossssss+`
      -osssssso.      :ssssssso.
     :osssssss/        osssso+++.
    /ossssssss/        +ssssooo/-
  `/ossssso+/:-        -:/+osssso+-
 `+sso+:-`                 `.-/+oso:
`++:.                           `-/+/
.`                                 `/

{}@{}
--------------------
OS: Arch Linux x86_64
Host: Discord
Kernel: 7.1.2-arch-rust
Uptime: {} mins
Packages: {} (pacman)
Shell: bash 5.3.9
Disk (/): 1.10 GiB / 1006.85 GiB (0%) - btrfs
Local IP (eth0): 123.456.789.0/20
Locale: {}.UTF-8
```",
                    ctx.author().name,
                    channel_name,
                    timestamp_secs,
                    userdata::pacman_counts(ctx.author().id.get()).await,
                    ctx.locale().unwrap()
                )),
        ),
    )
    .await?;
    Ok(())
}

/// Information about this bot
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let result = formatx!(
        i18n::get(ctx.locale().unwrap(), "about"),
        version = env!("CARGO_PKG_VERSION"),
        github = "https://github.com/ZiYueCommentary/archie-discord-bot",
        weblate = "https://weblate.ziyuesinicization.site/engage/archie-discord-bot/"
    )?;
    ctx.reply(result).await?;
    Ok(())
}
