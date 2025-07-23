If you can no longer flash your Bluepill with this error:

```
Running `probe-rs run --chip STM32F103CB --probe '0483:3748:60' /home/xxx/RustyBugA/target/thumbv7m-none-eabi/release/hello_world`
 WARN probe_rs::probe::stlink: send_jtag_command 242 failed: JtagNoDeviceConnected
 WARN probe_rs::probe::stlink: send_jtag_command 242 failed: JtagNoDeviceConnected
Error: Connecting to the chip was unsuccessful.

Caused by:
    0: An ARM specific error occurred.
    1: The debug probe encountered an error.
    2: An error which is specific to the debug probe in use occurred.
    3: Command failed with status JtagNoDeviceConnected.
Error: command exited with non-zero code `cargo run --release`: 1
```

try to boot the board [in BOOT mode](https://github.com/hmightypirate/AllMyCircuits?tab=readme-ov-file#flashing-the-firmware) and flash it again.

# References

- Suddenly unable to flash to STM32F407VGTx (STM32F407G-DISC1 ST-LINK V2.1): https://github.com/probe-rs/probe-rs/issues/300
- https://github.com/hmightypirate/AllMyCircuits?tab=readme-ov-file#flashing-the-firmware
