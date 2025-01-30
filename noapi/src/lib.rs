use clap::{Arg, Command};
use std::fs;
use std::path::Path;

fn generate_boilerplate(template_name: &str) -> std::io::Result<()> {
    let project_path = Path::new(template_name);

    // Create project directories
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("static"))?;
    fs::create_dir_all(project_path.join("static").join("assets"))?;

    // Create main.rs for the backend
    let main_rs = r#"
fn main() {
    println!("Hello, world!");
}
"#;
    fs::write(project_path.join("src").join("main.rs"), main_rs)?;

    // Create HTML file
    let index_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Project</title>
</head>
<body>
    <h1>Welcome to My Project</h1>
</body>
</html>
"#;
    fs::write(project_path.join("static").join("index.html"), index_html)?;

    // Create other necessary files (e.g., CSS, JS)
    let style_css = "body { font-family: Arial, sans-serif; }";
    fs::write(project_path.join("static").join("style.css"), style_css)?;

    let main_js = "console.log('Hello from JavaScript');";
    fs::write(project_path.join("static").join("main.js"), main_js)?;

    println!("Boilerplate generated at {:?}", project_path);
    Ok(())
}

fn main() {
    let matches = Command::new("perfect")
        .version("0.1.0")
        .author("Kelvin Osei")
        .about("Perfect web framework cli tool.")
        .arg(
            Arg::new("init")
                .short('i')
                .long("init")
                .value_name("PROJECT_NAME")
                .help("Generates a new project template"),
        )
        .get_matches();

    if let Some(project_name) = matches.value_of("init") {
        if let Err(e) = generate_boilerplate(project_name) {
            eprintln!("Error: {}", e);
        }
    } else {
        eprintln!("No project name provided. Use --init <PROJECT_NAME> to generate a project.");
    }
}
