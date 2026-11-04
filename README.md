# gameboy-learn-rs

My personal project for learning Game Boy development in Rust and building a game along the way.

Built with [rust-gb](https://github.com/zlfn/rust-gb) and
[rust-z80](https://github.com/llvm-z80/rust-z80), using SameBoy for emulation.

My one-time macOS setup for launching SameBoy with `cargo gb run`:

```sh
ln -s "$HOME/dev/gameboy-learn-rs/scripts/sameboy.sh" "$HOME/.cargo/bin/sameboy"
```
