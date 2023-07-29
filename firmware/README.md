# Hotkey keyboard firmware

## Building + Flashing

Requisites:

- thumbv6m-none-eabi Rust toolchain (install using `rustup target add thumbv6m-none-eabi`)
- [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)
- [dfu-util](https://dfu-util.sourceforge.net/)

1. Set keys and other parameters in [`parameters.rs`](src/parameters.rs)
2. Build using `cargo objcopy --release -- -O binary`
3. Flash by holding the BOOT0 key (the right one) while plugging in the keyboard and using `dfu-util -a 0 -d 0483:df11 -s 0x8000000:leave -D target/thumbv6m-none-eabi/release/hotkeyboard-firmware`

