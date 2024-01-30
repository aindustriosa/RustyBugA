# mightyBugA-rust-firmware
Board support crate for the MightyBugA board, based on
 - https://github.com/therealprof/nucleo-f103rb/tree/master
 - https://github.com/rubberduck203/stm32f3-discovery/tree/master


## Flashing
We use OpenOCD and GDB to connect to the board and flash the binary.

### Just flash and run the application
Check what runner you have uncommented in .cargo/config:
```
runner = "./flash.sh"
```
And build and flash an example (from the root):
```commandline
cargo xtask mightybuga_bsc example blink # use 'cargo xtask help' for a complete list of options
```
or from the BSC folder:


### Debugging
Check what runner you have uncommented in .cargo/config:

You need `runner = "gdb-multiarch -q -x openocd.gdb", start a openocd instance:

sudo openocd -f openocd.cfg

and:

cargo xtask mightybuga_bsc example blink
