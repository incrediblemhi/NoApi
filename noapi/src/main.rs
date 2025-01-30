mod boilerplate;

use clap::{arg, Command};

fn main() {
    let matches = Command::new("noapi")
        .version("0.1.0")
        .author("Kelvin Osei")
        .about("A Rust fullstack web framework [Axum + React], with the concept of Rust server actions, utilizes the type safety of Rust and TypeScript to make calls to server without APIs.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
            .short_flag('n')
                .about("Clones repos")
                .arg(arg!(project_name: "The name of your new project"))
                .arg_required_else_help(true),
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("new") {
        if let Some(project_name) = sub_matches.get_one::<String>("project_name") {
            if let Err(e) = boilerplate::generate_boilerplate(project_name) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
