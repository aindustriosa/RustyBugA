#![allow(dead_code)]
#![deny(unused_must_use)]

use std::{env, path::PathBuf};

use xshell::{cmd, Shell};

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    let mightybuga_bsc_example_names = get_mightybuga_bsc_example_names()?;

    match &args[..] {
        ["mightybuga_bsc", "example", example_name] => {
            // get if example_name is not in mightybuga_bsc_example_names
            if !mightybuga_bsc_example_names.contains(&example_name.to_string()) {
                println!("ERROR: example name {} is not in mightybuga_bsc_example_names", example_name);
                print_usage(mightybuga_bsc_example_names);
                return Ok(())
            }
            exec_mightybuga_bsc_example(example_name.to_string())},
        _ => {
            print_usage(mightybuga_bsc_example_names);
            Ok(())
        }
    }
}

fn print_usage(mightybuga_bsc_example_names: Vec<String>) {
    println!("USAGE:");
    for example_name in mightybuga_bsc_example_names {
        println!("\tcargo xtask mightybuga_bsc example {}", example_name);
    }
}

fn get_mightybuga_bsc_example_names() -> Result<Vec<String>, anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("mightybuga_bsc"));
    let output = cmd!(sh, "ls examples").read()?;
    let example_names = output.split("\n").map(|s| s[..s.len() - 3].to_string()).collect::<Vec<_>>();
    Ok(example_names)
}

fn exec_mightybuga_bsc_example(example_name: String) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("mightybuga_bsc"));
    cmd!(sh, "cargo run --example {example_name}").run()?;
    Ok(())
}


fn root_dir() -> PathBuf {
    let mut xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.pop();
    xtask_dir
}
