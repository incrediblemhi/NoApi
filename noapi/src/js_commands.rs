use std::{path::Path, process::Command};

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

pub fn npm_install() {
    if !Path::new("/node_modules").exists() {
        println!("Installing npm packages...");
        match Command::new(NPM).arg("install").status() {
            Ok(_status) => {}
            Err(error) => {
                eprintln!("{}", error)
            }
        };
    }
}
