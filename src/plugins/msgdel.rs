// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::{
    errors::UserBotError,
    userbot::{CommandHandlerResult, UserBot},
    *,
};
use grammers_client::types::{InputMessage, Message};
use reusable_fmt::fmt;
use std::cmp::min;

/// Delete a message
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !del`
///
//UserBotCmd !del
pub async fn msg_del(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let rep = bot
        .get_reply_to_message(message)
        .await
        .ok_or(UserBotError::NoArguments)?;
    let chat = message.chat();

    if bot.client.delete_messages(&chat, &[rep.id()]).await? != 0 {
        bot.client.delete_messages(&chat, &[message.id()]).await?;
        Ok(())
    } else {
        Err(UserBotError::MessageDeleteFailed)
    }
}

/// Delete multiple messages upto a certain message
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !purge`
///
//UserBotCmd !purge
pub async fn purge_msg(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let rep = bot
        .get_reply_to_message(message)
        .await
        .ok_or(UserBotError::NoArguments)?;
    let chat = message.chat();
    let mut futs = Vec::new();
    let mut msg_iter = bot.client.iter_messages(&chat);
    let mut ids = Vec::<i32>::new(); // List of ids until rep
    while let Some(msg) = msg_iter.next().await? {
        ids.push(msg.id());
        if msg.id() == rep.id() {
            break;
        }
    }

    loop {
        let chat_c = chat.clone();
        let handle_c = bot.client.clone();
        let col: Vec<i32> = ids.drain(..min(ids.len(), 100)).collect(); // Delete messages in a group of 100
        futs.push(tokio::spawn(async move {
            handle_c.delete_messages(&chat_c, &col).await.unwrap_or(0)
        }));
        if ids.is_empty() {
            break;
        }
    }

    let mut num_del: usize = 0;
    for fut in futs {
        num_del += fut.await.unwrap_or(0);
    }

    message
        .respond(InputMessage::markdown(fmt!(PURGED_TEMPLATE, num_del)))
        .await?;
    Ok(())
}
