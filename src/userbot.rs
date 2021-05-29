// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
//! High Level wrappers around protolib clients \
//! uses [`grammers_client`] in the back
use crate::{
    config::UserBotConfig,
    consts::{self, db},
    errors::{UserBotError, UserBotInitError},
//    handle_invoke,
    helpers::protolib as protohelper,
    plugins,
};
use grammers_client as gramme;
use grammers_mtsender::{InvocationError, ReadError};
use std::io;
use tokio::{
    io::BufReader,
    time::{sleep, Duration},
};

include!(concat!(env!("OUT_DIR"), "/commands.gen.rs"));

/// Expected Result returned from a command handler
pub type CommandHandlerResult = Result<(), UserBotError>;

/// Storage for objects that we reuse
///
/// [`grammers_client::Client`], [`mongodb::Database`], [`crate::config::UserBotConfig`]
pub struct UserBotStore {
    pub protoclient: gramme::Client,
    pub tg_id: i32,
    _mongo: mongodb::Client,
    pub db: mongodb::Database,
    pub conf: UserBotConfig,
}

impl UserBotStore {
    /// Create UserBotStore instance from [`crate::config::UserBotConfig`]
    pub async fn from_config(conf: UserBotConfig) -> Result<Self, UserBotInitError> {
        let mconf = mongodb::options::ClientOptions::parse(&conf.mongo.uri).await?;
        let mclient = mongodb::Client::with_options(mconf)?;
        let db = mclient.database(db::DB_NAME);

        let mut tclient = protohelper::client_from_config(&conf).await?;
        let me = tclient.get_me().await?;

        log::info!("{}", consts::BOT_READY);
        return Ok(Self {
            protoclient: tclient,
            tg_id: me.id(),
            _mongo: mclient,
            db: db,
            conf: conf,
        });
    }

    /// Wrapper around [`grammers_client::Client::next_updates`] \
    /// auto reconnects on ConnectionReset
    pub async fn next_update(&mut self) -> Option<gramme::Update> {
        let mut res;

        while {
            res = self.protoclient.next_update().await;
            res.is_err()
        } {
            let mut err: Option<&dyn std::error::Error> = None;
            let bind: Box<dyn std::error::Error>;
            match res.as_ref().err().unwrap() {
                InvocationError::Read(ReadError::Io(e)) => {
                    match e.kind() {
                        io::ErrorKind::ConnectionReset => {
                            // Reconnect
                            log::warn!("disconnected! trying to reconnect...");
                            let e = protohelper::client_from_config(&self.conf).await;
                            if let Err(e) = e {
                                bind = Box::new(e);
                                err = Some(bind.as_ref());
                            } else {
                                self.protoclient = e.unwrap();
                            }
                        }
                        _ => err = Some(e),
                    }
                }
                e => err = Some(e),
            }
            if let Some(e) = err {
                // Couldn't handle error
                log::error!("failed to fetch update, err: {}", e);
                break;
            }
        }

        return Some(res.ok()??);
    }

    fn save_session(&mut self) {
        log::info!("saving session...");
        if let Err(e) = self.protoclient.session().save_to_file(consts::SESSION_FILE) {
            log::warn!("failed to save session: {}", e);
        } else {
            log::info!("saved!");
        }
    }
}

impl Drop for UserBotStore {
    fn drop(&mut self) {
        self.save_session();
    }
}

/// A wrapper around [`grammers_client::ClientHandle`]
#[derive(Clone)]
pub struct UserBot {
    pub client: gramme::Client,
    pub tg_id: i32, // UserId cache
    pub db: mongodb::Database,
}

impl UserBot {
    /// Create UserBot instance from [`UserBotStore`]
    pub fn from_store(store: &UserBotStore) -> Self {
        Self {
            client: store.protoclient.clone(),
            tg_id: store.tg_id,
            db: store.db.clone(),
        }
    }

    /// Handle a single update from
    /// [`UserBotStore::next_updates`]
    pub async fn update_handler(
        &mut self,
        mut message: gramme::types::Message,
    ) -> Result<(), UserBotError> {
        if let Some(user_id) = message.sender().map(|u| u.id()) {
            if (user_id == self.tg_id || message.outgoing()) && message.text().starts_with('!') {
                if let Err(e) = command_handler(self, &mut message).await {
                    return self.command_err_handler(&mut message, e).await;
                }
            }
        }

        Ok(())
    }

    async fn command_err_handler(
        &self,
        message: &mut gramme::types::Message,
        e: UserBotError,
    ) -> Result<(), UserBotError> {
        use UserBotError::*;
        let msg: &str;
        let bind: String;
        let res = match e {
            MongoDB(_) | IO(_) | MessageForwardFailed => {
                // Most probably configuration problem
                msg = consts::BOT_CMD_FAIL;
                Err(e)
            }
            Invocation(inv) => match inv {
                InvocationError::Rpc(r) => {
                    // TODO: make errors easier to understand
                    bind = format!("err from telegram: `{}`", r.name);
                    msg = &bind;
                    Ok(())
                }
                _ => {
                    // Unhandled Error
                    msg = consts::BOT_CMD_FAIL;
                    Err(inv.into())
                }
            },
            _ => {
                // Most probably user's fault
                bind = e.to_string();
                msg = &bind;
                Ok(())
            }
        };

        message
            .edit(gramme::types::InputMessage::markdown(msg))
            .await?;
        return res;
    }

    /// The Message that the passed message is replying to,
    /// if any.
    pub async fn get_reply_to_message(
        &self,
        message: &gramme::types::Message,
    ) -> Option<gramme::types::Message> {
        return self
            .client
            .get_reply_to_message(message)
            .await
            .unwrap_or(None);
    }

    fn a_get_args(&self, msg: Option<&str>, split: bool) -> Result<Vec<String>, UserBotError> {
        if msg.is_none() {
            return Err(UserBotError::NoArguments);
        }
        let splitstr = msg.unwrap();

        let args: Vec<String> = if split {
            splitstr.split_whitespace().map(|s| s.to_string()).collect()
        } else {
            vec![splitstr.to_string()]
        };

        return Ok(args);
    }

    /// Get the message without the command prefix
    ///
    /// setting `split` splits the arguments beforehand
    pub fn get_args_nr(
        &self,
        message: &gramme::types::Message,
        split: bool,
    ) -> Result<Vec<String>, UserBotError> {
        let splitstr = message.text().splitn(2, " ").nth(1);
        return self.a_get_args(splitstr, split);
    }

    /// Get message without the command prefix
    ///
    /// setting `split` splits the argument beforehand
    ///
    /// If no arguments are found,
    /// the reply message is used as the argument, if any
    ///
    /// See [`UserBot::get_args_nr`] for a variant that ignores reply \
    /// See [`UserBot::get_arg_user`] for "User" related operations
    pub async fn get_args_text(
        &self,
        message: &gramme::types::Message,
        split: bool,
    ) -> Result<Vec<String>, UserBotError> {
        let repm: gramme::types::Message;
        let splitstr = if let Some(s) = message.text().splitn(2, " ").nth(1) {
            Some(s)
        } else if let Some(replym) = self.get_reply_to_message(message).await {
            repm = replym;
            Some(repm.text())
        } else {
            None
        };

        return self.a_get_args(splitstr, split);
    }

    /// Resolves the username in the message, if any \
    /// else returns the sender of the "reply-to" message, if any
    pub async fn get_arg_user(
        &mut self,
        message: &gramme::types::Message,
    ) -> Result<gramme::types::User, UserBotError> {
        let user = if let Ok(args) = self.get_args_nr(message, true) {
            let usrnm = if args[0].starts_with("@") {
                args[0].strip_prefix("@").unwrap()
            } else {
                &args[0]
            };

            self.client
                .resolve_username(usrnm)
                .await
                .map_err(|_| UserBotError::UserNotFound)?
                .ok_or(UserBotError::UserNotFound)
        } else if let Some(reply) = self.get_reply_to_message(message).await {
            reply.sender().ok_or(UserBotError::PeerNotUser)
        } else {
            Err(UserBotError::NoArguments)
        }?;

        return match user {
            gramme::types::Chat::User(u) => Ok(u),
            _ => Err(UserBotError::PeerNotUser),
        };
    }

    /// Wait for a new message in the chat
    /// requires the current latest message ⚠️[^w]
    ///
    /// [^w]: Assumes that passed message is the latest message
    pub async fn wait_reply(
        &self,
        chat: &gramme::types::Chat,
        prev: &gramme::types::Message,
    ) -> Result<(), UserBotError> {
        while self
            .client
            .iter_messages(chat)
            .next()
            .await?
            .map(|m| m.id() == prev.id())
            .unwrap_or(false)
        {
            sleep(Duration::from_millis(consts::REPLY_WAIT_TIME)).await;
        }

        Ok(())
    }

    pub async fn download_media(
        &self,
        media: &gramme::types::Media,
    ) -> Result<Vec<u8>, UserBotError> {
        let mut res_b = Vec::<u8>::new();
        let mut media_iter = self.client.iter_download(media);
        while let Some(chunk) = media_iter.next().await? {
            res_b.extend(chunk);
        }

        Ok(res_b)
    }

    pub async fn upload_media(
        &self,
        data: &[u8],
        fname: impl Into<String>,
    ) -> Result<gramme::types::media::Uploaded, UserBotError> {
        let sz = data.len();
        let mut data = BufReader::new(data);

        let doc = self
            .client
            .upload_stream(&mut data, sz, fname.into())
            .await?;
        Ok(doc.into())
    }
}
