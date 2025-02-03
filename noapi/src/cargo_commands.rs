use std::process::Command;

pub fn cargo_build() {
    match Command::new("cargo").arg("build").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn cargo_doc() {
    match Command::new("cargo").arg("doc").status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    };
}

pub fn cargo_install(tool: &str) {
    match Command::new("cargo").arg("install").arg(tool).status() {
        Ok(_status) => {}
        Err(error) => {
            eprintln!("{}", error)
        }
    }
}
