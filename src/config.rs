// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::consts;
use serde::Deserialize;
use std::{convert::Into, fs::File, io::Read};

/// Storage for Userbot's Config
#[derive(Deserialize)]
pub struct UserBotConfig {
    /// `optional`
    #[serde(default)]
    pub options: Options,
    /// `mandatory`
    pub telegram: TgConf,
    /// `mandatory`
    pub mongo: MongoConf,
}

#[derive(Deserialize)]
/// Storage for Userbot specific options,
pub struct Options {
    /// `optional` \
    /// Enable log file \
    /// Default: `true`
    #[serde(default = "Options::default_file_log")]
    pub file_log: bool,
}

/// Storage for telegram related configs
#[derive(Deserialize)]
pub struct TgConf {
    /// `mandatory` \
    /// Obtained from [https://my.telegram.org](https://my.telegram.org)
    pub api_id: i32,
    /// `mandatory` \
    /// Obtained from [https://my.telegram.org](https://my.telegram.org)
    pub api_hash: String,
    /// `mandatory` \
    /// `+` prefixed International phone number
    pub phone: String,
}

/// Storage for MongoDB related configs
#[derive(Deserialize)]
pub struct MongoConf {
    /// `mandatory` \
    /// Server url of mongodb server to connect to
    pub uri: String,
}

impl UserBotConfig {
    /// Parse `config.toml` file
    pub fn from_file() -> Result<UserBotConfig, std::io::Error> {
        let mut conf = File::open(consts::CONFIG_FILE)?;
        let mut confdata = String::new();
        conf.read_to_string(&mut confdata)?;
        toml::from_str(&confdata).map_err(Into::into)
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            file_log: Self::default_file_log(),
        }
    }
}

impl Options {
    fn default_file_log() -> bool {
        true
    }
}
