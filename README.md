# mightyBugA-rust-firmware
Firmware for the MightyBugA line follower done in Rust programming language

The design files for the main board can be found at: https://github.com/aindustriosa/RustyBugA-board


Tested on Ubuntu Linux

## Toolchain install
Installing:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Setup (old):
```
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7m-none-eabi
cargo install cargo-binutils cargo-embed cargo-flash cargo-expand
cargo install cargo-generate arm-toolchain
```

Setup (new, after probe-rs):
```
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7m-none-eabi
cargo install cargo-binutils cargo-expand
cargo install cargo-generate arm-toolchain
```

(Maybe this is not needed anymore)
```
wget https://armkeil.blob.core.windows.net/developer/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2

mkdir /home/$USER/arm-gcc/
tar -xf gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2 -C /home/$USER/arm-gcc/
```

## defmt and prove-rs dependencies
### flip-link:
```
cargo install flip-link
```
### probe-rs:
Install probe-rs by following the instructions at https://probe.rs/docs/getting-started/installation/.

## code docs
```
cargo doc --open
```

## Upload the firmware to the MCU
### Apps
#### Hello world
```commandline
DEFMT_LOG=trace cargo xtask run app hello_world
```

#### Line follower
```commandline
# With (semihosting) logs
DEFMT_LOG=trace cargo xtask run app line_follower
```

### BSC examples
```commandline
cargo xtask mightybuga_bsc example blink # use 'cargo xtask help' for a complete list of options

# With trace level logs
DEFMT_LOG=trace cargo xtask mightybuga_bsc example blink
```

## Debugging
Go to the [documentation for debugging](./docs/GDB-Debugging/gdb-debugging.md).

## References
Forked from https://cgit.pinealservo.com/BluePill_Rust/blue_pill_base

Other references:
 - https://github.com/punkto/rust_stm32f4_discovery_example/tree/main


## Sponsor

Esta actividade está patrocinada pola Xunta de Galicia e pola Axencia Para a Modernización Tecnolóxica (AMTEGA).
