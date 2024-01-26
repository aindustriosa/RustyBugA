# mightyBugA-rust-firmware
Firmware for the MightyBugA line follower done in Rust programming language


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

## Flashing
Check what runner you have uncommented in .cargo/config:
###
You need `runner = "gdb-multiarch -q -x openocd.gdb", start a openocd instance:
```commandline
sudo openocd -f openocd.cfg
```
and:
```commandline
cargo xtask mightybuga_bsc example blink # use 'cargo xtask help' for a complete list of options
```

### Use GDB to flash and start running the bin
You need `runner = "gdb-multiarch -q -x openocd_just_flash_and_run.gdb"`, start a openocd instance:
```commandline
sudo openocd -f openocd.cfg
```
and:
```commandline
cargo run
```
### Just use openocd to flash the binary:
You need `runner = "./flash.sh"` and:
```commandline
cargo run
```

### Unexpected idcode
The idcode refers to the chip identification. Real STM32F103C8T6s has idcode `0x2ba01477`, the CS32F103C8T6 clone has `0x1ba0147`. If openOCD exits with an error like this:
```
Warn : UNEXPECTED idcode: 0x1ba01477
Error: expected 1 of 1: 0x2ba01477
```
Update your chip id in openocd.cfg

References:
 - https://www.eevblog.com/forum/beginners/unexpected-idcode-flashing-bluepill-clone/
 - https://community.platformio.org/t/debugging-of-stm32f103-clone-bluepill-board-wrong-idcode/14635

## References
Forked from https://cgit.pinealservo.com/BluePill_Rust/blue_pill_base

Other references:
 - https://github.com/punkto/rust_stm32f4_discovery_example/tree/main
