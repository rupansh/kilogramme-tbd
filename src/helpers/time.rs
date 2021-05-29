// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use crate::consts::time;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Parse a duration string of custom format
///
/// Format:
///
/// nd: n is a whole number and d signifies days \
/// nh: n is a whole number and h signifies hours \
/// nm: n is a whole number and m signifies minutes
///
/// # Examples
/// `5d` -- 5 days \
/// `30h` -- 30 hours \
/// `34m` -- 34 minutes
pub fn parse_duration(dur: &str) -> Option<Duration> {
    let time_c: u64 = match dur.chars().last() {
        Some('d') => time::DAY_SECS,
        Some('h') => time::HOUR_SECS,
        Some('m') => time::MINUTE_SECS,
        _ => return None,
    };

    let dur = dur.get(..dur.len() - 1)?.parse::<u64>().ok()?;

    return Some(Duration::from_secs(dur * time_c));
}

/// Current system time in UNIX time format
///
/// panics if time stamp is somehow set before 1970
pub fn epoch_ms() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("IMPOSSIBLE thanos.png")
        .as_millis();
}
