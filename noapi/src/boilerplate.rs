use std::{fs, path::Path};

pub fn generate_boilerplate(project_name: &str) -> std::io::Result<()> {
    let project_path = Path::new(project_name);

    // Create project directories
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("static"))?;
    fs::create_dir_all(project_path.join("static").join("assets"))?;

    // Create main.rs for the backend

    fs::write(project_path.join("src").join("main.rs"), MAIN_RS)?;

    // Create HTML file

    fs::write(project_path.join("static").join("index.html"), INDEX_HTML)?;

    // Create other necessary files (e.g., CSS, JS)
    let style_css = "body { font-family: Arial, sans-serif; }";
    fs::write(project_path.join("static").join("style.css"), style_css)?;

    let main_js = "console.log('Hello from JavaScript');";
    fs::write(project_path.join("static").join("main.js"), main_js)?;

    println!("Boilerplate generated at {:?}", project_path);
    Ok(())
}

const MAIN_RS: &str = r#"
fn main() {
    println!("Hello, world!");
}
"#;

const INDEX_HTML: &str = r#"
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
