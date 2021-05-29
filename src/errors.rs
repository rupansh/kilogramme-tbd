// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use grammers_client::SignInError;
use grammers_mtsender::{AuthorizationError, InvocationError, ReadError};
use std::io;
use thiserror::Error;

/// Main Error Wrapper
///
/// should be used only while bot is handling updates
#[derive(Error, Debug)]
pub enum UserBotError {
    #[error("`{0}`")]
    Invocation(#[from] InvocationError),
    #[error("`{0}`")]
    MongoDB(#[from] mongodb::error::Error),
    #[error("`{0}`")]
    IO(#[from] io::Error),
    #[error("`{0}`")]
    Image(#[from] image::ImageError),
    #[error("`required arguments not provided!`")]
    NoArguments,
    #[error("`couldn't get chat (report to dev)`")]
    ChatNotFound,
    #[error("`peer is not user (report to dev)`")]
    PeerNotUser,
    #[error("`failed to forward message`")]
    MessageForwardFailed,
    #[error("`note does not exist!`")]
    NoteNotFound,
    #[error("`no notes added`")]
    NotesEmpty,
    #[error("`failed to delete message`")]
    MessageDeleteFailed,
    #[error("`user not found`")]
    UserNotFound,
    #[error("`given string is not a valid emoji`")]
    StrNotEmoji,
    #[error("`given message doesn't have a sticker`")]
    NoSticker,
}

/// Initialization Error Wrapper
///
/// should be used only while the bot is initializing
#[derive(Error, Debug)]
pub enum UserBotInitError {
    #[error("`{0}`")]
    Invocation(#[from] InvocationError),
    #[error("`{0}`")]
    Authorization(#[from] AuthorizationError),
    #[error("`{0}`")]
    Read(#[from] ReadError),
    #[error("`{0}`")]
    SignIn(#[from] SignInError),
    #[error("`{0}`")]
    MongoDB(#[from] mongodb::error::Error),
    #[error("`{0}`")]
    IO(#[from] io::Error),
}
