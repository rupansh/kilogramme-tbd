// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::{config::UserBotConfig, consts, errors::UserBotInitError};
use grammers_client::{Client, Config};
use grammers_session::Session;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::{self, AsyncBufReadExt, BufReader};

fn new_session(path: &Path) -> io::Result<Session> {
    let session = Session::new();
    session.save_to_file(path)?;

    Ok(session)
}

async fn load_or_create_session() -> io::Result<Session> {
    let session_path = Path::new(consts::SESSION_FILE);
    let meta = fs::metadata(session_path).await;

    if meta.is_err() {
        File::create(session_path).await?;
        new_session(session_path)
    } else if meta.unwrap().len() == 0 {
        new_session(session_path)
    } else {
        Session::load_file(session_path)
    }
}

/// Create an instance of [`grammers_client::Client`] from [`crate::config::UserBotConfig`]
///
/// uses [`grammers_session::Session`] as its session type
pub async fn client_from_config(conf: &UserBotConfig) -> Result<Client, UserBotInitError> {
    let tconf = Config {
        session: load_or_create_session().await?,
        api_id: conf.telegram.api_id,
        api_hash: conf.telegram.api_hash.clone(),
        params: Default::default(),
    };

    let mut client = Client::connect(tconf).await?;
    if !client.is_authorized().await? {
        let sent_code = client
            .request_login_code(
                &conf.telegram.phone,
                conf.telegram.api_id,
                &conf.telegram.api_hash,
            )
            .await?;
        println!("Please Enter Authentication from Telegram");
        let mut code = String::new();
        BufReader::new(io::stdin()).read_line(&mut code).await?;
        client.sign_in(&sent_code, &code).await?;
        client.session().save_to_file(consts::SESSION_FILE)?;
    }

    return Ok(client);
}
