mod database;
mod userdata;
mod commands;
mod utils;
mod i18n;

use poise::{serenity_prelude as serenity};
use tracing::{info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};
use std::env;
use tracing::log::error;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let appender = tracing_appender::rolling::daily("logs", "archie");
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(appender);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("error,archie_discord_bot=debug"));

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(true)
        .with_line_number(true);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_writer)
        .with_target(true)
        .with_line_number(true);

    Registry::default()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();
    i18n::init();

    match database::init().await {
        Err(e) => error!("Can't initialize database: {e:?}"),
        _ => info!("Database initialized successfully")
    }

    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::fastfetch(), commands::nyaofetch(), commands::pacman::pacman(), commands::ask::ask(), commands::about()],
            ..poise::FrameworkOptions::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if let Err(why) = client.unwrap().start().await {
        error!("Client error: {why:?}");
    }
}
