use std::process::Command;

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

pub fn npm_install() {
    println!("Installing npm packages...");
    match Command::new(NPM).arg("install").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn yarn_install() {
    println!("Installing node modules...");
    match Command::new("yarn").arg("install").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn pnpm_install() {
    println!("Installing node modules...");
    match Command::new("pnpm").arg("install").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn bun_install() {
    println!("Installing bun modules...");
    match Command::new("bun").arg("install").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn deno_install() {
    println!("Installing deno packages...");
    match Command::new("deno").arg("install").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}
