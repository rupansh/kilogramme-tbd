// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
/// Constants related to Databse
pub mod db;
/// info plugin constants
pub mod info;
/// manage plugin constants
pub mod manage;
/// memes plugin constants (and static regexes)
pub mod memes;
/// msgdel plugin constants
pub mod msgdel;
/// notes plugin constants
pub mod notes;
/// ⚠️***WARN***⚠️: NSFW
pub mod nsfw;
/// restrict plugin constants
pub mod restrict;
/// sticker plugin constants
pub mod stickers;
/// time helper constants
pub mod time;

/// Location of log file
pub const LOG_FILE: &str = "userbot.log";

/// Location of config file
pub const CONFIG_FILE: &str = "config.toml";
/// Location of session file
pub const SESSION_FILE: &str = "userbot.session";

pub const BOT_READY: &str = "USERBOT IS READY!!";
pub const BOT_CMD_FAIL: &str = "Failed to execute command, logged";

pub const HELLO_TEXT: &str = "`Hi from KiloGramme (TBD)`";

// 100ms
pub const REPLY_WAIT_TIME: u64 = 100;
