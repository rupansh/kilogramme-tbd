// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::{
    consts::restrict::*,
    helpers::time::parse_duration,
    userbot::{CommandHandlerResult, UserBot},
};
use grammers_client::types::{Chat, InputMessage, Message};
use std::time::Duration;

/// Ban a user
///
/// ⚠️***WARN***⚠️: Kicks user if not in super group
///
/// ## Scope
/// Group Chat
///
/// ## Usage(s)
/// `<reply> !ban <time>` \
/// `!ban @username <time>`
///
/// ## Example(s)
/// `!ban @test 12d` \
/// `!ban @test`
///
//UserBotCmd !ban
pub async fn ban_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let time_arg = bot
        .get_args_nr(message, true)
        .ok()
        .and_then(|a| a.last().map(|s| parse_duration(s)))
        .flatten();

    let b_user = bot.get_arg_user(message).await?;
    let chat = message.chat();

    let msg: &str;
    if matches!(chat, Chat::Channel(_)) {
        bot.client
            .set_banned_rights(&chat, &b_user)
            .view_messages(false)
            .duration(time_arg.unwrap_or(Duration::from_secs(0)))
            .await?;
        msg = BAN_TXT;
    } else {
        bot.client.kick_participant(&chat, &b_user).await?;
        msg = BAN_WARN;
    }

    message.edit(InputMessage::markdown(msg)).await?;
    Ok(())
}

/// Unban a user
///
/// ## Scope
/// Supergroup
///
/// ## Usage(s)
/// `<reply> !unban` \
/// `!unban @username`
///
//UserBotCmd !unban
pub async fn unban_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let ub_user = bot.get_arg_user(message).await?;

    bot.client
        .set_banned_rights(&message.chat(), &ub_user)
        .await?;

    message.edit(InputMessage::markdown(UNBAN_TXT)).await?;
    Ok(())
}

/// Kick a user
///
/// ## Scope
/// Group Chat
///
/// ## Usage(s)
/// `<reply> !kick` \
/// `!kick @username`
///
//UserBotCmd !kick
pub async fn kick_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let k_user = bot.get_arg_user(message).await?;

    bot.client
        .kick_participant(&message.chat(), &k_user)
        .await?;

    message.edit(InputMessage::markdown(KICK_TXT)).await?;
    Ok(())
}

/// Mutes a user
///
/// ⚠️***WARN***⚠️: Kicks the user if not in supergroup
///
/// ## Scope
/// Group Chat
///
/// ## Usage(s)
/// `<reply> !mute` \
/// `!mute @username`
///
//UserBotCmd !mute
pub async fn mute_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let time_arg = bot
        .get_args_nr(message, true)
        .ok()
        .and_then(|a| a.last().map(|s| parse_duration(s)))
        .flatten();

    let m_user = bot.get_arg_user(message).await?;

    bot.client
        .set_banned_rights(&message.chat(), &m_user)
        .send_messages(false)
        .duration(time_arg.unwrap_or(Duration::from_secs(0)))
        .await?;

    message.edit(InputMessage::markdown(MUTE_TXT)).await?;
    Ok(())
}

/// Unmutes a user
///
/// Equivalent of [`unban_handler`]
///
/// ## Scope
/// Supergroup
///
/// ## Usage(s)
/// `<reply> !unmute` \
/// `!unmute @username`
///
//UserBotCmd !unmute
pub async fn unmute_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    unban_handler(bot, message).await
}
