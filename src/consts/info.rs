// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use reusable_fmt::fmt_reuse;

fmt_reuse! {
    // This has 2 spaces at the end of every line to force line breaks
    INFO_TEMPLATE = r#"{header}\
**ID:** {id}  
**First Name:** {fname}  
**Last Name:** {lname}  
**Username:** @{username}"#;

    USER_ID_FMT = "**UserID:** `{}`";
    CHAT_ID_FMT = "**CHATID:** `{}`";
    P_UID_FMT = "***Personal UID:** `{}`";

    PONG_FMT = r#"***PONG!***\
`Ping: {}ms`"#;
}

pub const BOT_HEADER: &str = "**Bot Info**";
pub const USER_HEADER: &str = "**User Info**";

pub const PONG: &str = "***PONG!***";
