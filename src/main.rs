// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
mod config;
mod consts;
/// High level Error related wrapper Enums
mod errors;
mod helpers;
/// Command Handlers
mod plugins;
mod userbot;

use config::UserBotConfig;
use ctrlc;
use grammers_client::Update;
use helpers::time;
use simplelog::*;
use std::fs::File;
use userbot::*;
use tokio::sync::watch;

// TODO: Documentation, Kang
fn main() {
    let config = UserBotConfig::from_file();
    if config.is_err() {
        log::error!("invalid config!! {}", config.err().unwrap());
        return;
    }
    let config = config.unwrap();

    // Initialize Logging
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];
    if config.options.file_log {
        loggers.push(WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(consts::LOG_FILE).unwrap(),
        ))
    }

    simplelog::CombinedLogger::init(loggers).unwrap();

    log::info!("started logging @ {}", time::epoch_ms());

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main(config));

    log::info!("bot stopped @{}", time::epoch_ms());
}

async fn async_main(config: UserBotConfig) {
    // Initialize userbot
    let store = UserBotStore::from_config(config).await;
    if let Err(e) = store {
        log::error!("failed to initialize bot {}", e);
        return;
    }
    let mut store = store.unwrap();

    let (tx, mut rx) = watch::channel::<()>(());

    if ctrlc::set_handler(move || {
        log::info!("shutting down bot...");
        tx.send(()).unwrap();
    }).is_err() {
        log::warn!("failed to set ctrl+c handler. Bot won't save session before quitting!");
    }

    // Start fetching updates
    while let Some(update) = tokio::select! {
        biased;
        res = rx.changed() => {
            res.unwrap();
            return;
        },
        update = store.next_update() => {
            update
        }
    } {
        let bot = UserBot::from_store(&store);
        if let Update::NewMessage(message) = update {
            let mut bot = bot.clone();
            tokio::task::spawn(async move {
                if let Err(e) = bot.update_handler(message).await {
                    log::warn!("error handling update: {}", e);
                }
            });
        }
    }
}
