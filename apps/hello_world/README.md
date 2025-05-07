# Hello world

Basic app that uses libs and BSC to print something and make demos

## How to run
From this folder:
```commandline
apps/hello_world$ cargo run --bin hello_world

# With logs
apps/hello_world$ DEFMT_LOG=trace cargo run --bin hello_world
```

from the root folder using xtask
```commandline
DEFMT_LOG=trace cargo xtask run app hello_world
```
