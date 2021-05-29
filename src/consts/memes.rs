// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use lazy_static::lazy_static;
use regex::Regex;

// OwO
pub const FACES: [&str; 15] = [
    "(・`ω´・)",
    ";;w;;",
    "owo",
    "UwU",
    ">w<",
    "^w^",
    "( ^ _ ^)∠☆",
    "(ô_ô)",
    "~:o",
    ";____;",
    "(*^*)",
    "(>_<)",
    "(♥_♥)",
    "*(^O^)*",
    "((+_+))",
];

type OwoRep = (Regex, &'static str);
lazy_static! {
    pub static ref RLW1: OwoRep = (Regex::new(r"(ｒ|ｌ)").unwrap(), "ｗ");
    pub static ref RLW2: OwoRep = (Regex::new(r"(R|L)").unwrap(), "W");
    pub static ref RLW3: OwoRep = (Regex::new(r"(Ｒ|Ｌ)").unwrap(), "Ｗ");
    pub static ref RLW4: OwoRep = (Regex::new(r"(r|l)").unwrap(), "w");
    pub static ref NYR1: OwoRep = (Regex::new(r"n([aeiouａｅｉｏｕ])").unwrap(), r"ny$1");
    pub static ref NYR2: OwoRep = (Regex::new(r"ｎ([ａｅｉｏｕ])").unwrap(), r"ｎｙ$1");
    pub static ref NYR3: OwoRep = (Regex::new(r"N([aeiouAEIOU])").unwrap(), r"Ny$1");
    pub static ref NYR4: OwoRep = (
        Regex::new(r"Ｎ([ａｅｉｏｕＡＥＩＯＵ])").unwrap(),
        r"Ｎｙ$1"
    );
    pub static ref FACE_REG1: Regex = Regex::new(r"!+").unwrap();
}
pub const OVEUV1: (&str, &str) = ("ove", "uv");
pub const OVEUV2: (&str, &str) = ("ｏｖｅ", "ｕｖ");

// Zalgo
pub const ZALG_TOP: [&str; 46] = [
    " ̍", " ̎", " ̄", " ̅", " ̿", " ̑", " ̆", " ̐", " ͒", " ͗", " ͑", " ̇", " ̈", " ̊", " ͂", " ̓", " ̈́", " ͊", " ͋",
    " ͌", " ̃", " ̂", " ̌", " ͐", " ́", " ̋", " ̏", " ̽", " ̉", " ͣ", " ͤ", " ͥ", " ͦ", " ͧ", " ͨ", " ͩ", " ͪ", " ͫ",
    " ͬ", " ͭ", " ͮ", " ͯ", " ̾", " ͛", " ͆", " ̚",
];
pub const ZALG_MID: [&str; 21] = [
    " ̕", " ̛", " ̀", " ́", " ͘", " ̡", " ̢", " ̧", " ̨", " ̴", " ̵", " ̶", " ͜", " ͝", " ͞", " ͟", " ͠", " ͢", " ̸",
    " ̷", " ͡",
];
pub const ZALG_BOT: [&str; 39] = [
    " ̗", " ̘", " ̙", " ̜", " ̝", " ̞", " ̟", " ̠", " ̤", " ̥", " ̦", " ̩", " ̪", " ̫", " ̬", " ̭", " ̮", " ̯", " ̰",
    " ̱", " ̲", " ̳", " ̹", " ̺", " ̻", " ̼", " ͅ", " ͇", " ͈", " ͉", " ͍", " ͎", " ͓", " ͔", " ͕", " ͖", " ͙", " ͚",
    " ",
];

// Stretch
lazy_static! {
    pub static ref STRETCH_REG: Regex = Regex::new(r"([aeiouAEIOUａｅｉｏｕＡＥＩＯＵ])").unwrap();
}
