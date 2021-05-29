// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use reusable_fmt::fmt_reuse;

fmt_reuse! {
    PACK_NAME_FMT = "p{id}_by_{user}_{time}";
    PACK_TITLE_FMT = "{user}'s Very Nice Kang Pack";
    PACK_ADD_SUCCESS = "Sticker added to [pack](t.me/addstickers/{pack_name})";
}

pub const DEFAULT_EMOJI: &str = "🌚";
pub const STICKERS_USERNAME: &str = "Stickers";
pub const ANON_USER: &str = "anonymous";
pub const CREATING_PACK: &str = "`Kang pack does not exist! Creating...`";
pub const STICKER_FILE: &str = "sticker.png";
pub const STICKERSET_INVALID: &str = "STICKERSET_INVALID";
