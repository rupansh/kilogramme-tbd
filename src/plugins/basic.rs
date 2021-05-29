// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::consts::HELLO_TEXT;
use crate::userbot::{CommandHandlerResult, UserBot};
use grammers_client::types::{InputMessage, Message};

/// Edits the command message and greets you
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s):
/// `!hello`
///
//UserBotCmd !hello
pub async fn hello(_bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    message.edit(InputMessage::markdown(HELLO_TEXT)).await?;
    Ok(())
}
