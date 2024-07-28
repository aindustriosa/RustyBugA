# mightyBugA-rust-firmware
Firmware for the MightyBugA line follower done in Rust programming language

The design files for the main board can be found at: https://github.com/aindustriosa/RustyBugA-board


Tested on Ubuntu Linux

## Toolchain install
Installing:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Setup:
```
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7m-none-eabi
cargo install cargo-binutils cargo-embed cargo-flash cargo-expand
cargo install cargo-generate arm-toolchain
```

```
wget https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2

mkdir /home/$USER/arm-gcc/
tar -xf gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2 -C /home/$USER/arm-gcc/
```

## code docs
```
cargo doc --open
```

## Upload the firmware to the MCU

```commandline
cargo xtask mightybuga_bsc example blink # use 'cargo xtask help' for a complete list of options
```

Set the environment variable *UPLOAD_TOOL* (default is `openocd`) to use your prefered tool for uploading the firmware to the flash memory of the MCU:

| tools | *UPLOAD_TOOL* | notes |
|-------|---------------|-------|
| OpenOCD + STLink-v2 probe | openocd | This is the default. |
| BlackMagicProbe | blackmagic | *BMP_PORT* variable to set the device where the BMP is connected (default is `/dev/ttyACM0`). |

Example to use BMP:

```sh
export UPLOAD_TOOL=blackmagic
cargo xtask run app hello_world
```

## Debugging
Go to the [documentation for debugging](./docs/GDB_Debugging/gdb-debugging.md).

## References
Forked from https://cgit.pinealservo.com/BluePill_Rust/blue_pill_base

Other references:
 - https://github.com/punkto/rust_stm32f4_discovery_example/tree/main


## Sponsor

Esta actividade está patrocinada pola Xunta de Galicia e pola Axencia Para a Modernización Tecnolóxica (AMTEGA).
