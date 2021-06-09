# Kilogramme (TBD)

[![RUST](https://img.shields.io/badge/made%20with-RUST-red.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![MPL-2.0](https://img.shields.io/badge/license%20-MPL--2.0-white.svg?style=for-the-badge&logo=mozilla)](https://spdx.org/licenses/MPL-2.0.html)
[![DOCS](https://img.shields.io/github/workflow/status/rupansh/kilogramme-tbd/docs?style=for-the-badge&logo=read-the-docs&logoColor=white&label=docs)](https://rupansh.github.io/kilogramme-tbd/kilogramme_tbd/)

*High Performance MTProto Telegram Userbot*

**WIP**

## Prerequisites
- [mongoDB](https://docs.mongodb.com/manual/installation/) server
- [Rust](https://www.rust-lang.org/) 2018 Edition
- [Telegram](https://telegram.org/) Account

## Usage

```shell
cargo build --release
./target/release/kilogramme-tbd
```

## Configuration

See [config.example.toml](config.example.toml) for an example config

Refer to [docs/CONFIG.md](docs/CONFIG.md) for an in-depth overview

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)

## FAQ

> Why rust?

rust is based and I wanted proper static typing  
> Can I use this in production?

lol  
> how's the performance?

yes

## To-do

Sorted by priority

- More documentation (available commands, etc)
- `Dockerfile`
- Fixing sticker related commands
- Many memez
- Battle testing (#helpme)

## Other stuff to remember

The source code has some not so safe for work stuff. It has been marked as such with a warning.  
tread with caution.
