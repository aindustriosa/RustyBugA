NOTE: With the adoption of probe-rs, we do not launch gdb and have no rules for debugging (go to [this commit](https://github.com/aindustriosa/RustyBugA/tree/5a5e5e1400c02fc40d0ed10d5d8124af1fe18f25) for a version of the code with these capabilities).
TODO: Follow https://probe.rs/docs/tools/debugger and document how to debug with current tools.

# GDB debugging
Note that, currently, the debug commands are done inside each binary folder. This is because xtask only has rules for release (not debug) binaries and because each application folder has its own `.cargo` folder with the how to run aliases. 

## Step 1: Config cargo to use GDB with the correct OpenOCD config file for debugging:
In `mightybuga_bsc/.cargo/config` or `apps/{APP_NAME}/.cargo/config`, depending on what you want to debug, you need to uncomment this line:
```
runner = "gdb-multiarch -q -x openocd_debug.gdb"
```

## Step 2: Start a OpenOCD instance
Then start a openocd instance (from the directory of an app or `mightybuga_bsc`), for example:

```bash
cd mightybuga_bsc # or apps/{APP_NAME}
sudo openocd -f openocd.cfg
```

## Step 3: Compile and run GDB
In the app's directory:

```bash
cd mightybuga_bsc # or apps/{APP_NAME}
cargo run # you can also debug an example with `--example {EXAMPLE_NAME}`
```


# GDB debugging with VSCode
We have had success debugging in VSCode using the [Cortex-Debug plugin](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug) and [the Device Support Pack - STM32F1](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug-dp-stm32f1) that launches OpenOCD and GDB from the IDE using a `launch.json` like this for the "hello_world" app:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Cortex Debug",
            "cwd": "${workspaceFolder}",
            "executable": "./target/thumbv7m-none-eabi/debug/hello_world",
            "request": "launch",
            "type": "cortex-debug",
            "runToEntryPoint": "main",
            "servertype": "openocd",
            "configFiles": [
                "./apps/hello_world/openocd.cfg"
            ],
            "showDevDebugOutput": "raw",
            "deviceName": "STM32F103C8"
        }
    ]
}
```
