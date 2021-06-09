## Creating new commands

See [src/plugins/basic.rs](../src/plugins/basic.rs) for a "Hello World" of sorts.

Implementing a new command is trivial. 

A command should more or less follow this format:

```rust
//UserBotCmd !newcommand
pub async fn something_new(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult {
	// Do stuff
	Ok(())
}
```

Eventually, instructions for adding command documentation will be added as well.

Lets break this down.

#### `//UserBotCmd !newcommand`:
This line is important. Think of `//UserBotCmd` like a proc macro (or a decorator if you are from python).  
This tells tells the [build script](../build.rs) necessary information about the command.  
You don't need to do anything else! The runtime will know when to handle this command! (i.e when you, the owner sends `!newcommand` message through a telegram client)  
Ensure that "newcommand" does not conflict with existing commands.

#### `pub`:
Only command handlers should be public in a plugin file!  
If you need common functionality across multiple plugins, consider adding it to the [helpers](../src/helpers) module.

#### `async fn`:
Command handlers should be asynchronous. Avoid adding blocking code.  
If you really need it, check [tokio::task::spawn_blocking](https://docs.rs/tokio/1/tokio/task/fn.spawn_blocking.html).

#### `something_new`:
There are no limitations to the function identifier. If its valid in rust, its valid for the bot.

#### `(bot: &mut UserBot, message: &mut Message) -> CommandHandlerResult`:
The function should follow this signature. (`bot` & `message` aren't mandatory identifier names, but recommended)

`bot` is a [UserBot](https://rupansh.github.io/kilogramme-tbd/kilogramme_tbd/userbot/struct.UserBot.html]) instance.  
You can use various helper functions provided, along with a direct instance to `grammers_client::Client` & `mongodb::Database`.  
`mongodb::Database` might eventually become private.  
The aim of this helper is to provide extra functionality over `grammers_client::Client` and not replace it.

`message` is the command message sent by the owner.

`CommandHandlerResult` is nothing but `Result<(), UserBotError>`.  
See [src/errors.rs](../src/errors.rs).  
You may add more Error variants to `UserBotError` if required

## Creating a new plugin

A plugin is just a simple module with responsibilities related to a set of tasks that have something in common (It could be their functionality, scope, usage, etc).

All plugins reside in the [plugins](../src/plugins) module.

To create a new plugin, create a new file in [plugins](../src/plugins) module.

Update [src/plugins/mod.rs](../src/plugins/mod.rs).

If your new plugin is `newplugin.rs`,
the following lines must be added to the `mod.rs` file:

```rust
/// Short description of the plugin(mandatory)
mod newplugin;
```

```rust
pub use newplugin::*;
```

All plugin files must begin with the

```
// Copyright 2021 - 2021, <Plugin Auth> and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
```
header.
