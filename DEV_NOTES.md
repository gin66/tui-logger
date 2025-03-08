# Release process

## Prepare documentation

Run first `cargo rdme` and then decide on `cargo rdme --force`

## Update Changelog and Cargo.toml

Execute along the lines of:
1. `release-plz update`, then check in the versioned files.
2. `git push`
3. wait for github runners are completed
4. `git tag` and `git push --tags`
5. `cargo publish`

# Update demo.gif

In `doc` folder run `vhs demo.tape`.
Then rename `demo.gif` to current version and update Readme - currently via lib.rs

There is another `demo-short.tape`, which is used for the demo in ratatui website.

# Needed tools on macos

```sh
cargo install release-plz
cargo install cargo-rdme
brew install vhs
```
