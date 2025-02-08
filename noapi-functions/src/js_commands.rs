use std::process::{Command, ExitStatus};

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(windows)]
const YARN: &str = "yarn.cmd";
#[cfg(windows)]
const PNPM: &str = "pnpm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";
#[cfg(not(windows))]
const YARN: &str = "yarn";
#[cfg(not(windows))]
const PNPM: &str = "pnpm.cmd";

pub fn npm_build() -> Result<ExitStatus, std::io::Error> {
    Command::new(NPM).arg("run").arg("build").status()
}

pub fn yarn_build() -> Result<ExitStatus, std::io::Error> {
    Command::new(YARN).arg("run").arg("build").status()
}

pub fn pnpm_build() -> Result<ExitStatus, std::io::Error> {
    Command::new(PNPM).arg("run").arg("build").status()
}

pub fn bun_build() -> Result<ExitStatus, std::io::Error> {
    Command::new("bun").arg("run").arg("build").status()
}

pub fn deno_build() -> Result<ExitStatus, std::io::Error> {
    Command::new("deno").arg("run").arg("build").status()
}
