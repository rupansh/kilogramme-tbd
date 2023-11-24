// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::consts::{memes::*, nsfw};
use crate::userbot::{CommandHandlerResult, UserBot};
use grammers_client::types::{InputMessage, Message};
use rand::seq::SliceRandom;
use rand::Rng;

use std::fmt::Write;

/// owo text (・`ω´・)
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !owo` \
/// `!owo some-text`
///
//UserBotCmd !owo
pub async fn owo_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let arg = &bot.get_args_text(message, false).await?[0];

    let res = RLW1.0.replace_all(arg, RLW1.1);
    let res = RLW2.0.replace_all(&res, RLW2.1);
    let res = RLW3.0.replace_all(&res, RLW3.1);
    let res = NYR1.0.replace_all(&res, NYR1.1);
    let res = NYR2.0.replace_all(&res, NYR2.1);
    let res = NYR3.0.replace_all(&res, NYR3.1);
    let res = NYR4.0.replace_all(&res, NYR4.1);
    let rand_f = format!(" {}", *FACES.choose(&mut rand::thread_rng()).unwrap());
    let res = FACE_REG1.replace_all(&res, rand_f.as_str());
    let res = res.replace(OVEUV1.0, OVEUV1.1);
    let res = res.replace(OVEUV2.0, OVEUV2.1);
    let res = format!(
        "{} {}",
        &res,
        *FACES.choose(&mut rand::thread_rng()).unwrap()
    );

    message.edit(InputMessage::text(res)).await?;
    Ok(())
}

/// streeeeetch teeeeext
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !stretch` \
/// `!stretch some-text`
///
//UserBotCmd !stretch
pub async fn stretch_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let arg = &bot.get_args_text(message, false).await?[0];

    let numr = r"${1}".repeat(rand::thread_rng().gen_range(3..10));
    let res = STRETCH_REG.replace_all(arg, numr.as_str());

    message.edit(InputMessage::text(res)).await?;
    Ok(())
}

/// ｖａｐｏｒ  ｔｅｘｔ
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !vapor` \
/// `!vapor some-text`
///
//UserBotCmd !vapor
pub async fn vapor_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let arg = &bot.get_args_text(message, false).await?[0];

    let res: String = arg.as_str().chars().fold(String::new(), |mut out, charac| {
        let _ = write!(
            out,
            "{}{}",
            if charac.is_whitespace() { " " } else { "" },
            if 0x21 <= charac as u32 && charac as u32 <= 0x7F {
                std::char::from_u32(charac as u32 + 0xFEE0)
                    .unwrap_or(std::char::REPLACEMENT_CHARACTER)
            } else if charac as u32 == 0x20 {
                std::char::from_u32(0x3000).unwrap()
            } else {
                charac
            }
        );
        out
    });

    message.edit(InputMessage::text(res)).await?;
    Ok(())
}

/// lE ePic MocK tExT
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !mock` \
/// `!mock some-text`
///
//UserBotCmd !mock
pub async fn mock_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let arg = &bot.get_args_text(message, false).await?[0];

    let res: String = arg
        .as_str()
        .chars()
        .map(|charac| {
            if charac.is_alphabetic() && rand::thread_rng().gen_range(0..2) == 1 {
                if charac.is_lowercase() {
                    charac.to_uppercase().to_string()
                } else {
                    charac.to_lowercase().to_string()
                }
            } else {
                charac.to_string()
            }
        })
        .collect();

    message.edit(InputMessage::text(res)).await?;
    Ok(())
}

fn rand_zalg() -> String {
    let mut rng = rand::thread_rng();

    return match rng.gen_range(0..3) {
        0 => ZALG_TOP.choose(&mut rng).unwrap(),
        1 => ZALG_BOT.choose(&mut rng).unwrap(),
        _ => ZALG_MID.choose(&mut rng).unwrap(),
    }
    .trim()
    .to_string();
}

/// z͎̋a̭̋l͓̕gͤ́o̫̬ t̀͛ e̞͗x̜͡t͓ͬ
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `<reply> !zalgo` \
/// `!zalgo some-text`
///
//UserBotCmd !zalgo
pub async fn zalgo_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let arg = &bot.get_args_text(message, false).await?[0];

    let res: String = arg
        .as_str()
        .chars()
        .map(|charac| {
            if charac.is_alphabetic() {
                format!("{}{}{}{}", charac, rand_zalg(), rand_zalg(), rand_zalg())
            } else {
                charac.to_string()
            }
        })
        .collect();

    message.edit(InputMessage::text(res)).await?;
    Ok(())
}

/// ⚠️***WARN***⚠️: NSFW
///
/// v.funny human phallus.txt
///
/// ## Scope
/// Anywhere
///
/// ## Usage(s)
/// `!pp <n>`
///
/// ## Example(s)
/// `!pp 10` \
/// `!pp`
//UserBotCmd !pp
pub async fn pp_handler(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
    let n = bot
        .get_args_nr(message, false)
        .map(|v| v[0].parse::<usize>().ok())
        .ok()
        .flatten()
        .unwrap_or(1usize);

    message.edit(InputMessage::text(nsfw::PP.repeat(n))).await?;
    Ok(())
}
