// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::{
    consts::info::*,
    errors::UserBotError,
    userbot::{CommandHandlerResult, UserBot},
    *,
};
use grammers_client::types::{Chat, InputMessage, Message};
use reusable_fmt::fmt;

/// Get chat id or id of the sender of "reply-to" message
///
/// ## Scope
/// Group Chat \
/// Private Message
///
/// ## Usage(s)
/// `<reply> !id` \
/// `!id`
///
//UserBotCmd !id
pub async fn id_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let res: String;

    res = match bot.get_arg_user(message).await {
        Ok(user) => fmt!(USER_ID_FMT, user.id()),
        Err(UserBotError::NoArguments) => {
            let chat = message.chat();
            match chat {
                Chat::User(_) => fmt!(P_UID_FMT, bot.tg_id),
                _ => fmt!(CHAT_ID_FMT, chat.id()),
            }
        }
        Err(e) => return Err(e.into()),
    };

    message.edit(InputMessage::markdown(res)).await?;
    Ok(())
}

/// Get basic information about the chat or about the sender of "reply-to" message
///
/// ## Scope
/// Group Chat \
/// Private Message
///
/// ## Usage(s)
/// `<reply> !info` \
/// `!info`
///
//UserBotCmd !info
pub async fn info_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let user = match bot.get_arg_user(message).await {
        Ok(user) => user,
        Err(UserBotError::NoArguments) => match message.sender() {
            Some(Chat::User(u)) => u,
            _ => Err(UserBotError::PeerNotUser)?,
        },
        Err(e) => return Err(e.into()),
    };

    let infos = fmt!(
        INFO_TEMPLATE,
        header = if user.is_bot() {
            BOT_HEADER
        } else {
            USER_HEADER
        },
        id = user.id(),
        fname = user.first_name(),
        lname = user.last_name().unwrap_or("N/A"),
        username = user.username().unwrap_or("N/A"),
    );

    message.edit(InputMessage::markdown(infos)).await?;
    Ok(())
}

/// Get the connection speed to telegram server
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `!ping`
///
//UserBotCmd !ping
pub async fn ping_handler(_bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    use std::time::Instant;

    let now = Instant::now();
    message.edit(InputMessage::markdown(PONG)).await?;
    let elapsed = now.elapsed();

    let els = fmt!(PONG_FMT, elapsed.as_millis());

    message.edit(InputMessage::markdown(els)).await?;
    Ok(())
}
