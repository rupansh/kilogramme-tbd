// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::{
    errors::UserBotError,
    userbot::{CommandHandlerResult, UserBot},
    *,
};
use grammers_client::types::{Chat, InputMessage, Message};
use reusable_fmt::fmt;

/// Creates a Note
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !note note-key`
///
//UserBotCmd !note
pub async fn add_note_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let name = &bot.get_args_nr(message, true)?[0];
    let rep_id = message
        .reply_to_message_id()
        .ok_or(UserBotError::NoArguments)?;
    // TODO: I NEED PEERSELF HERE!!!
    let me = bot.client.get_me().await?;
    let fwd = bot
        .client
        .forward_messages(&Chat::User(me), &[rep_id], &message.chat())
        .await?;

    let fwd = fwd[0].as_ref().ok_or(UserBotError::MessageForwardFailed)?;

    helpers::db::add_note(&bot.db, name, fwd.id()).await?;

    message
        .edit(InputMessage::markdown(&fmt!(NOTE_SUCCESS, name)))
        .await?;
    Ok(())
}

/// Fetch a Note
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `!get note-key`
///
//UserBotCmd !get
pub async fn get_note_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let name = bot.get_args_nr(message, true)?.remove(0);
    let note = helpers::db::find_note(&bot.db, &name).await?.ok_or(UserBotError::NoteNotFound)?;

    // TODO: I NEED PEERSELF HERE!!!
    let me = bot.client.get_me().await?;
    bot.client
        .forward_messages(
            &message.chat(),
            &[note],
            &Chat::User(me),
        )
        .await?;

    Ok(())
}

/// Delete a Note
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `!clear note-key`
///
//UserBotCmd !clear
pub async fn rm_notes_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let name = bot.get_args_nr(message, true)?.remove(0);

    if helpers::db::remove_note(&bot.db, &name).await?.deleted_count != 0 {
        message
            .edit(InputMessage::markdown(fmt!(NOTE_DEL_SUCCESS, name)))
            .await?;
        Ok(())
    } else {
        return Err(UserBotError::NoteNotFound);
    }
}

/// Get a list of all note keys
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `!notes`
///
//UserBotCmd !notes
pub async fn all_notes_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let keys: Vec<String> = helpers::db::note_list(&bot.db).await?.collect();
    if keys.len() > 0 {
        message
            .edit(InputMessage::markdown(keys.join("\n"),))
            .await?;
        Ok(())
    } else {
        return Err(UserBotError::NotesEmpty);
    }
}
