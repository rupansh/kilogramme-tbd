// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::{
    consts::manage::*,
    errors::UserBotError,
    userbot::{CommandHandlerResult, UserBot},
};
use grammers_client::types::{InputMessage, Message};

/// pin "reply-to" message
///
/// ## Scope
/// Group Chat
///
/// ## Usage(s)
/// `<reply> !pin`
///
//UserBotCmd !pin
pub async fn pin_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let reply_id = message
        .reply_to_message_id()
        .ok_or(UserBotError::NoArguments)?;
    bot.client.pin_message(&message.chat(), reply_id).await?;

    message.edit(InputMessage::markdown(PIN_TXT)).await?;
    Ok(())
}

/// unpin "reply-to" message
///
/// ## Scope
/// Group Chat
///
/// ## Usage(s)
/// `<reply> !unpin`
///
//UserBotCmd !unpin
pub async fn unpin_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    bot.client.unpin_all_messages(&message.chat()).await?;

    message.edit(InputMessage::markdown(UNPIN_TXT)).await?;
    Ok(())
}

/// promote a user to admin
///
/// ## Scope
/// Group Chat
///
/// ## Usage(s)
/// `<reply> !promote` \
/// `!promote @username`
///
//UserBotCmd !promote
pub async fn promote_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let user = bot.get_arg_user(message).await?;

    bot.client
        .set_admin_rights(&message.chat(), &user)
        .load_current()
        .await?
        .pin_messages(true)
        .delete_messages(true)
        .ban_users(true)
        .invite_users(true)
        .await?;

    message.edit(InputMessage::markdown(PROMOTE_TXT)).await?;
    Ok(())
}

/// demote an admin
///
/// ## Scope
/// Group Chat
///
/// ## Usage(s)
/// `<reply> !demote` \
/// `!demote @username`
///
//UserBotCmd !demote
pub async fn demote_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let chat = message.chat();
    let user = bot.get_arg_user(message).await?;

    bot.client.set_admin_rights(&chat, &user).await?;

    message.edit(InputMessage::markdown(DEMOTE_TXT)).await?;
    Ok(())
}
