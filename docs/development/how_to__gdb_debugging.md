# GDB Debug

You need the line

```
runner = "gdb-multiarch -q -x openocd_debug.gdb"
```

in `mightybuga_bsc/.cargo/config` or `apps/{APP_NAME}/.cargo/config`.

Then start a openocd instance (from the directory of an app or `mightybuga_bsc`):

```bash
sudo openocd -f openocd.cfg
```

and in the app's directory:

```bash
cargo run
```

or you can use an `xtask` command to debug an example.

