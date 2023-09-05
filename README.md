# rust-scratchpad

An example of using `cargo workspace` and [snapshot testing](https://docs.rs/insta/latest/insta/) to create a REPL-like experience.

## Requirements

`cargo install insta`

## How I use this template

1. Develop in "library" crates in the workspaces, keep their dependency lean.
2. Make an "application" crate that depends on all the crates you developed in 1. Heavy dependencies (such as tokio) should only exist in this crate.
3. Explore your code in the `scratchpad` crate. Use [insta](https://docs.rs/insta/latest/insta/) for inline snapshot to evaluate and print results directly in the code.
4. If there's anything worth keeping, make it a test in library crates.

## Running snapshot tests for scratchpad via insta

Typically, you want to do inline snapshot test for the `scratchpad` crate with the command `cargo insta test -p scratchpad --accept`.

You can invoke the command by:

1. in vim: `:!make`
2. in clion: setup a custom cargo run configuration for the command
3. via `cargo-watch`: `cargo watch -x 'insta test -p scratchpad --accept'`