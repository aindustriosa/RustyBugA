#![allow(dead_code)]
#![deny(unused_must_use)]

use std::{env, path::PathBuf};

use xshell::{cmd, Shell};

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    let mightybuga_bsc_example_names = get_mightybuga_bsc_example_names()?;
    let lib_names = get_lib_names()?;
    let app_names = get_app_names()?;

    match &args[..] {
        ["mightybuga_bsc", "example", example_name] => {
            if !mightybuga_bsc_example_names.contains(&example_name.to_string()) {
                println!(
                    "ERROR: example name {} is not in mightybuga_bsc_example_names",
                    example_name
                );
                print_usage(mightybuga_bsc_example_names, lib_names, app_names);
                return Ok(());
            }
            exec_mightybuga_bsc_example(example_name.to_string())
        }
        ["test", "lib", lib_name] => {
            if !lib_names.contains(&lib_name.to_string()) {
                println!("ERROR: lib name {} is not in lib_names", lib_name);
                print_usage(mightybuga_bsc_example_names, lib_names, app_names);
                return Ok(());
            }
            exec_test_lib(lib_name)
        },
        ["run", "app", app_name] => {
            if !app_names.contains(&app_name.to_string()) {
                println!("ERROR: app name {} is not in app_names", app_name);
                print_usage(mightybuga_bsc_example_names, lib_names, app_names);
                return Ok(());
            }
            exec_run_app_release(app_name)
        },
        _ => {
            print_usage(mightybuga_bsc_example_names, lib_names, app_names);
            Ok(())
        }
    }
}

fn print_usage(mightybuga_bsc_example_names: Vec<String>, lib_names: Vec<String>, app_names: Vec<String>) {
    println!("USAGE:");
    for example_name in mightybuga_bsc_example_names {
        println!("\tcargo xtask mightybuga_bsc example {}", example_name);
    }
    for lib_name in lib_names {
        println!("\tcargo xtask test lib {}", lib_name);
    }
    for app_name in app_names {
        println!("\tcargo xtask run app {}", app_name);
    }
}

fn get_mightybuga_bsc_example_names() -> Result<Vec<String>, anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("mightybuga_bsc"));
    let output = cmd!(sh, "ls examples").read()?;
    let example_names = output
        .split("\n")
        .map(|s| s[..s.len() - 3].to_string())
        .collect::<Vec<_>>();
    Ok(example_names)
}

fn get_lib_names() -> Result<Vec<String>, anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("libs"));
    let output = cmd!(sh, "ls").read()?;
    let lib_names = output.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    Ok(lib_names)
}

fn get_app_names() -> Result<Vec<String>, anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("apps"));
    let output = cmd!(sh, "ls").read()?;
    let app_names = output.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    Ok(app_names)
}

fn exec_mightybuga_bsc_example(example_name: String) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("mightybuga_bsc"));
    cmd!(sh, "cargo run --example {example_name}").run()?;
    Ok(())
}

fn exec_test_lib(lib_name: &&str) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("libs").join(lib_name));
    cmd!(sh, "cargo test").run()?;
    Ok(())
}

fn exec_run_app_release(app_name: &&str) -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;
    sh.change_dir(root_dir().join("apps").join(app_name));
    cmd!(sh, "cargo run --release").run()?;
    Ok(())
}

fn root_dir() -> PathBuf {
    let mut xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.pop();
    xtask_dir
}
