# mightyBugA-rust-firmware
Board support crate for the MightyBugA board, based on
 - https://github.com/therealprof/nucleo-f103rb/tree/master
 - https://github.com/rubberduck203/stm32f3-discovery/tree/master


## Examples
### Building
```commandline
# build an example with dev profile
cargo build --example blink

# build an example with dev profile and trace level logs
DEFMT_LOG=trace cargo build --example blink
```

### Flashing (running)
```commandline
# build an example with dev profile and trace level logs
DEFMT_LOG=trace cargo run --example blink
```

Note that these logs are printed through the semihosting interface (the JTAG interface, not the serial port).

## Debugging
Go to the [documentation for debugging](../docs/GDB-Debugging/gdb-debugging.md).

