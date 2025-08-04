# Vaersion of the Hello world that uses lin*app_tests_menu

Basic app that uses libs and BSC to print something and make demos

## How to run
From this folder:
```commandline
apps/tests_menu$ cargo run --bin tests_menu

# With logs
apps/tests_menu$ DEFMT_LOG=trace cargo run --bin tests_menu
```

from the root folder using xtask
```commandline
DEFMT_LOG=trace cargo xtask run app tests_menu
```
