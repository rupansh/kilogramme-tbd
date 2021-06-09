# Contributing to this repository

## Prerequisites

This project is licensed under [MPL-2.0](https://spdx.org/licenses/MPL-2.0.html). Your contribution should be, too.

All code should be pre-formatted with `cargo fmt`

usage of `unsafe` is discouraged, but not restricted. If there's a safe alternative (even with minor performance degradation), it should be preferred.

Nightly code is not allowed, unless used in docs or tests.

Check [gramme.rs] bleeding edge docs. This project uses the github repo instead of the crates.io release.

Consider going through the [api docs](https://rupansh.github.io/kilogramme-tbd/kilogramme_tbd/).

## Code Quality Improvements

Since the project is in infancy, there's plenty of scope for code improvements.  
Consider checking the [helpers](../src/helpers) module!  
Check [src/plugins/memes.rs](../src/plugins/memes.rs) && [src/consts/memes.rs](../src/consts/memes.rs).

## New Dependencies

There must be a good reason to add a new dependency.  
Refrain from adding a library if what you need is trivial to implement manually.

## Adding/Improving Functionality

There are plenty of routes to go if you wish to add/improve functionality of this bot.


### New Commands/Plugins

See [CONTRIB-COMMANDS.md](./CONTRIB-COMMANDS.md)

### Improving existing plugins

Changing the function identifier or changing the command name does not count as "code improvement" and is rather, a [design improvement](#other-ways-to-contribute).

Feel free to add performance improvements, code improvements, etc though.

### `UserBot` helper improvements

Remember that `UserBot` is just a wrapper over `grammers_client::Client`.
Think if the improvement is applicable to `grammers_client::Client` as well. If so, contribute to the [gramme.rs] project!

## Other ways to contribute

- Feature Requests: Feel free to ask for features by creating a new issue.
- Design Improvements: Any improvements to the existing design (Command Names, Structure of the project, etc) count as design improvement. Create a new issue.
- Contribute to [gramme.rs] : This project heavily relies on grammers. Any improvements to [gramme.rs] will most likely benefit this project as well! 


[gramme.rs]: https://github.com/Lonami/grammers/
