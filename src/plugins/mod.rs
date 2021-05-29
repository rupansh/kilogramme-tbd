// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
/// Example bot command handler
mod basic;
/// Handlers related to Userbot & Telgram Info
mod info;
/// Handlers related to group management
///
/// ⚠️***WARN***⚠️: We do not explicitly check permissions. However we can easily handle permission errors.
/// handling erros is faster than explicitly checking permissions!
mod manage;
/// le epic memes
///
/// currently text-related only
mod memes;
/// Handlers related to message manipulation
mod msgdel;
/// Handlers related to saving and deleting notes
mod notes;
/// Handlers related to restricting users
///
/// ⚠️***WARN***⚠️: We do not explicitly check permissions. However we can easily handle permission errors.
/// handling erros is faster than explicitly checking permissions!
mod restrict;
/// Kangz and stuffz
mod stickers;

pub use basic::*;
pub use info::*;
pub use manage::*;
pub use memes::*;
pub use msgdel::*;
pub use notes::*;
pub use restrict::*;
pub use stickers::*;
