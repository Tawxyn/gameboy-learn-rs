# gameboy-learn-rs

A Game Boy project based on rust-gb's minimal template.

Edit `src/main.rs` for your game and `header.toml` for the cartridge settings.

```sh
cargo gb build
cargo gb run
```

The ROM is written to `target/gameboy-learn-rs.gb`. Running requires an emulator
(`sameboy` by default, or the command set in `EMULATOR`).

This project uses the registered `rust-z80` toolchain and the installed `cargo-gb`
command. Its library dependencies point to the sibling `../rust-gb` checkout.
Keep that checkout and the linked toolchain at `../rust-z80/install` available.
