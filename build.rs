// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use reusable_fmt::{fmt, fmt_reuse};
use std::{
    ffi::OsStr,
    fs,
    io::{self, prelude::*, BufReader, BufWriter},
    path::Path,
};

fmt_reuse! {
    CMD_GEN_SRC = r#"
pub async fn command_handler(bot: &mut UserBot, message: &mut gramme::types::Message) -> Result<(), UserBotError> {{
    return match message.text().split_whitespace().next() {{
        {src}
        _ => Ok(()),
    }}
}}

"#;
}

const COMMAND_PREFIX: &str = "//UserBotCmd";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=src/plugins/*.rs");

    let ext = Some(OsStr::new("rs"));
    let files = fs::read_dir("./src/plugins/")?
        .map(|r| r.map(|e| e.path()))
        .filter_map(|p| p.ok())
        .filter(|p| p.file_name().unwrap() != "mod.rs" && p.extension() == ext);

    let mut cases = Vec::<String>::new();
    for file in files {
        let srcf = fs::File::open(file)?;
        let reader = BufReader::new(srcf);

        let mut fn_flag = false;
        let mut prev_cmd = String::new();
        for line in reader.lines().filter_map(|p| p.ok()) {
            if let Some(c) = line.trim().strip_prefix(COMMAND_PREFIX) {
                fn_flag = true;
                prev_cmd = c.trim().to_string();
            } else if fn_flag {
                // Remove the prefix first, extract function identifier
                // Can be improved probably
                let fn_name = line
                    .trim()
                    .strip_prefix("pub async fn")
                    .unwrap()
                    .trim()
                    .split('(')
                    .next()
                    .unwrap()
                    .split('<')
                    .next()
                    .unwrap();

                cases.push(format!(
                    "Some(\"{prev_cmd}\") => plugins::{fn_name}(bot, message).await,"
                ));
                fn_flag = false;
            }
        }
    }

    let cmd_hnd_src = fmt!(CMD_GEN_SRC, src = &cases.join("\n"));
    let cmd_hnd_file = Path::new(&std::env::var("OUT_DIR").unwrap()).join("commands.gen.rs");
    let mut writer = BufWriter::new(fs::File::create(cmd_hnd_file).unwrap());
    write!(&mut writer, "{cmd_hnd_src}").unwrap();

    Ok(())
}
