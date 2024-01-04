#![allow(dead_code)]
#![deny(unused_must_use)]

use std::{env, path::PathBuf};

use xshell::{cmd, Shell};

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    match &args[..] {
        ["mightybuga_bsc", "example", "blink"] => exec_mightybuga_bsc_example_blink(),
        ["mightybuga_bsc", "example", "usart_no_bsc"] => exec_mightybuga_bsc_example_usart_no_bsc(),
        ["mightybuga_bsc", "example", "usart1_echo"] => exec_mightybuga_bsc_example_usart1_echo(),
        _ => {
            println!("USAGE:");
            println!("\tcargo xtask mightybuga_bsc example blink");
            println!("\tcargo xtask mightybuga_bsc example usart_no_bsc");
            println!("\tcargo xtask mightybuga_bsc example usart1_echo");
            Ok(())
        }
    }
}

fn exec_mightybuga_bsc_example_usart1_echo() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("mightybuga_bsc"));
    cmd!(sh, "cargo run --example usart1_echo").run()?;
    Ok(())
}

fn exec_mightybuga_bsc_example_blink() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("mightybuga_bsc"));
    cmd!(sh, "cargo run --example blink").run()?;
    Ok(())
}

fn exec_mightybuga_bsc_example_usart_no_bsc() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("mightybuga_bsc"));
    cmd!(sh, "cargo run --example usart_no_bsc").run()?;
    Ok(())
}

fn root_dir() -> PathBuf {
    let mut xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.pop();
    xtask_dir
}
