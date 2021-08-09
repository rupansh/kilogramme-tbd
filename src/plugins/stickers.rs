// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::{
    consts::{stickers::*},
    errors::UserBotError,
    helpers::image,
    userbot::{CommandHandlerResult, UserBot},
    *,
};
use grammers_client::types::{Chat, InputMessage, Media, Message};
use grammers_mtproto::mtp::RpcError;
use grammers_mtsender::InvocationError;
use grammers_tl_types as tl;
use once_cell::sync::OnceCell;
use reusable_fmt::fmt;
use std::ops::Deref;
use tokio::sync::Mutex;
use unic_emoji_char::is_emoji;

// This Mutex prevents concurrent kangs
static STICKER_CHAT: OnceCell<Mutex<Chat>> = OnceCell::new();

/// Download a sticker as a file
///
/// ⚠️***WARN***⚠️: Currently can't handle stickers on other Data Centers
///
/// ## Scope
/// Anywhere
///
/// Usage(s): \
/// `<reply> !getsticker`
///
//UserBotCmd !getsticker
pub async fn get_sticker_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let reply = bot
        .get_reply_to_message(message)
        .await
        .ok_or(UserBotError::NoArguments)?;

    let media = reply.media().ok_or(UserBotError::NoSticker)?;
    let sticker_file = match media {
        Media::Sticker(sticker) => sticker.document,
        Media::Document(document) if document.mime_type() == Some("image/webp") => document,
        _ => Err(UserBotError::NoSticker)?,
    };

    let fname = sticker_file.name().to_string();
    let sticker_media = Media::Document(sticker_file);

    let dlbuf = bot.download_media(&sticker_media).await?;
    let sticker = bot.upload_media(&dlbuf, fname).await?;

    message.reply(InputMessage::text("").file(sticker)).await?;

    Ok(())
}

async fn pack_exists(bot: &mut UserBot, pack: &str) -> Result<bool, InvocationError> {
    let stickerset = tl::types::InputStickerSetShortName {
        short_name: pack.to_string()
    };

    let req = tl::functions::messages::GetStickerSet {
        stickerset: stickerset.into()
    };

    match bot.client.invoke(&req).await {
        Ok(_) => Ok(true),
        Err(InvocationError::Rpc(RpcError { ref name, .. })) if name == STICKERSET_INVALID => Ok(false),
        Err(e) => Err(e.into()),
    }
}

/// Duplicate a sticker into a userbot's custom sticker pack
///
/// ⚠️***WARN***⚠️: Currently can't handle stickers on other Data Centers
///
/// ## Scope
/// Anywhere
///
/// Usage(s): \
/// `<reply> !kang <emoji>`
///
/// Example(s): \
/// `<reply> !kang 😁` \
/// `<reply> !kang`
///
// TODO: eventually use Bot token
//UserBotCmd !kang
pub async fn kang_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let args = bot.get_args_nr(message, true);
    let reply = bot
        .get_reply_to_message(message)
        .await
        .ok_or(UserBotError::NoArguments)?;

    let mut emoji = DEFAULT_EMOJI.to_string();
    let mut resize = true;
    let media = reply.media().ok_or(UserBotError::NoArguments)?;
    let sticker = match &media {
        Media::Photo(_) => Ok(media),
        Media::Sticker(sticker) => {
            emoji = sticker.emoji().to_string();
            resize = false;
            Ok(media)
        }
        Media::Document(document) if document.mime_type().filter(|mime| matches!(*mime, "image/webp" | "image/png" | "image/jpeg")).is_some() => Ok(media),
        _ => Err(UserBotError::NoArguments),
    }?;

    if let Ok(mut args) = args {
        emoji = args.remove(0);
        if !is_emoji(emoji.chars().next().unwrap()) {
            return Err(UserBotError::StrNotEmoji);
        }
    }

    let me = bot.client.get_me().await?;
    let pack_user = me.username().unwrap_or(ANON_USER);

    let pack = helpers::db::sticker_pack_name(&bot.db, bot.tg_id, pack_user).await?;
    let pack_title = fmt!(PACK_TITLE_FMT, user = pack_user);

    let mut sticker_message = InputMessage::text("");
    sticker_message = if resize {
        let stick_bytes = bot.download_media(&sticker).await?;
        let im: Vec<u8> = tokio::task::spawn_blocking(move || {
            let im = image::image_from_buf(&stick_bytes)?;
            // TG stickers are 512x512
            let im = image::im_resize_clamped(&im, 512, 512);
            image::png_encode(im)
        }).await.unwrap()?;
        let sticker = bot.upload_media(&im, STICKER_FILE).await?;
        sticker_message.document(sticker)
    } else {
        sticker_message.copy_media(&sticker)
    };

    let mut sticker_chat = STICKER_CHAT.get();
    if sticker_chat.is_none() {
        let chat = bot
            .client
            .resolve_username(STICKERS_USERNAME)
            .await?
            .ok_or(UserBotError::ChatNotFound)?;
        STICKER_CHAT.set(Mutex::new(chat)).unwrap();
        sticker_chat = STICKER_CHAT.get();
    }

    let chatg = sticker_chat.unwrap().lock().await;
    let chat: &Chat = chatg.deref();

    let mut prev = bot
        .client
        .send_message(chat, InputMessage::text("/cancel"))
        .await?;
    bot.wait_reply(chat, &prev).await?;

    if !pack_exists(bot, &pack).await? {
        // Create pack
        message.edit(InputMessage::markdown(CREATING_PACK)).await?;

        prev = prev.respond(InputMessage::text("/newpack")).await?;
        bot.wait_reply(chat, &prev).await?;

        prev = prev.respond(InputMessage::text(pack_title)).await?;
        bot.wait_reply(chat, &prev).await?;

        // Send Sticker
        prev = prev
            .respond(sticker_message)
            .await?;
        bot.wait_reply(chat, &prev).await?;

        prev = prev.respond(InputMessage::text(emoji)).await?;
        bot.wait_reply(chat, &prev).await?;

        prev = prev.respond(InputMessage::text("/publish")).await?;
        bot.wait_reply(chat, &prev).await?;

        prev = prev.respond(InputMessage::text("/skip")).await?;
        bot.wait_reply(chat, &prev).await?;

        // Pack name
        prev = prev.respond(InputMessage::text(&pack)).await?;
        bot.wait_reply(chat, &prev).await?;
    } else {
        prev = prev.respond(InputMessage::text("/addsticker")).await?;
        bot.wait_reply(chat, &prev).await?;

        prev = prev.respond(InputMessage::text(&pack)).await?;
        bot.wait_reply(chat, &prev).await?;

        // Send Sticker
        prev = prev
            .respond(sticker_message)
            .await?;
        bot.wait_reply(chat, &prev).await?;

        prev = prev.respond(InputMessage::text(emoji)).await?;
        bot.wait_reply(chat, &prev).await?;

        prev = prev.respond(InputMessage::text("/done")).await?;
        bot.wait_reply(chat, &prev).await?;
    }

    // static test to ensure that the Mutex isn't dropped before
    // the kang process is complete
    std::mem::drop(chatg);

    message
        .edit(InputMessage::markdown(fmt!(PACK_ADD_SUCCESS, pack_name = pack)))
        .await?;

    Ok(())
}
