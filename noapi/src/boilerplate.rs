use std::{fs, path::Path};

pub fn generate_boilerplate(project_name: &str) -> std::io::Result<()> {
    let package_json: &str = &format!(
        r#"
    {{
  "name": "{}",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {{
    "dev": "vite",
    "build": "tsc -b && vite build",
    "lint": "eslint .",
    "preview": "vite preview"
}},
  "dependencies": {{
    "@tailwindcss/vite": "^4.0.0",
    "axios": "^1.7.9",
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "react-router-dom": "^7.1.5"
}},
  "devDependencies": {{
    "@eslint/js": "^9.17.0",
    "@types/node": "^22.13.0",
    "@types/react": "^18.3.18",
    "@types/react-dom": "^18.3.5",
    "@vitejs/plugin-react": "^4.3.4",
    "eslint": "^9.17.0",
    "eslint-plugin-react-hooks": "^5.0.0",
    "eslint-plugin-react-refresh": "^0.4.16",
    "globals": "^15.14.0",
    "tailwindcss": "^4.0.0",
    "typescript": "~5.6.2",
    "typescript-eslint": "^8.18.2",
    "vite": "^6.0.5"
}}
}}
"#,
        project_name
    );

    let cargo_toml: &str = &format!(
        r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8.1"
listenfd = "1.0.2"
tokio = {{version = "1.43.0", features = ["full"]}}
tower = "0.5.2"
tower-http = {{ version = "0.6.2", features = ["fs"] }}
regex = "1.11.1"
noapi-functions = "0.1.0"
serde = {{version = "1.0.217", features = ["derive"]}}
serde_json = "1.0.138"
tower-livereload = "0.9.6"

[build-dependencies]
noapi-functions = "0.1.1"
"#,
        project_name
    );

    let project_path = Path::new(project_name);

    // Create project directories
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("src").join("handlers"))?;
    fs::create_dir_all(project_path.join("frontend"))?;
    fs::create_dir_all(project_path.join("frontend").join("pages"))?;

    // src files
    fs::write(project_path.join("src").join("main.rs"), MAIN_RS)?;
    fs::write(project_path.join("src").join("functions.rs"), FUNCTIONS_RS)?;

    // src/handlers files
    fs::write(project_path.join("src").join("handlers").join("mod.rs"), "")?;
    fs::write(
        project_path
            .join("src")
            .join("handlers")
            .join("functions.rs"),
        "",
    )?;

    // frontend files
    fs::write(project_path.join("frontend").join("index.css"), "")?;
    fs::write(project_path.join("frontend").join("index.html"), INDEX_HTML)?;
    fs::write(project_path.join("frontend").join("main.tsx"), MAIN_TSX)?;
    fs::write(
        project_path.join("frontend").join("vite-env.d.ts"),
        VITE_ENV,
    )?;

    // frontend/pages files
    fs::write(
        project_path
            .join("frontend")
            .join("pages")
            .join("index.tsx"),
        INDEX_TSX,
    )?;
    fs::write(
        project_path.join("frontend").join("pages").join("404.tsx"),
        ERROR_TSX,
    )?;

    // root files
    fs::write(project_path.join(".gitignore"), GITIGNORE)?;
    fs::write(project_path.join("build.rs"), BUILD_RS)?;
    fs::write(project_path.join("Cargo.toml"), cargo_toml)?;
    fs::write(project_path.join("eslint.config.js"), ESLINT_CONFIG)?;
    fs::write(project_path.join("functions.ts"), "")?;
    fs::write(project_path.join("package.json"), package_json)?;
    fs::write(project_path.join("README.md"), README)?;
    fs::write(project_path.join("tailwind.config.js"), TAILWIND_CONFIG)?;
    fs::write(project_path.join("tsconfig.app.json"), TSCONFIG_APP)?;
    fs::write(project_path.join("tsconfig.json"), TSCONFIG)?;
    fs::write(project_path.join("tsconfig.node.json"), TSCONFIG_NODE)?;
    fs::write(project_path.join("vite.config.ts"), VITE_CONFIG)?;

    println!("New NoApi project generated at {:?}", project_path);

    Ok(())
}

const MAIN_RS: &str = r#"
pub mod functions;
pub mod handlers;

use handlers::create_router;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower_livereload::LiveReloadLayer;
// imports from cargo spaces
use noapi_functions::generate_routes_from_folder;

const STATIC_DIR: &str = "./src/static";

#[tokio::main]
async fn main() {
    let app = create_router();

    let app = generate_routes_from_folder(STATIC_DIR, app);

    app.layer(LiveReloadLayer::new());

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

"#;

const FUNCTIONS_RS: &str = r#"
pub fn add(num1: u32, num2: u32) -> u32 {
    num1 + num2
}
"#;

const INDEX_TSX: &str = r#"
import { add } from "@functions";
import { useState } from "react";

function App() {
  let [result, setResult] = useState(0);
  let [num, setNum] = useState(0);

  return (
    <>
      <main>
        <h1
          onClick={() => {
            add(num, result).then((res) => {
              setResult(res);
            });
          }}
          className="font-semibold text-2xl"
        >
          {result}
        </h1>
      </main>
    </>
  );
}

export default App;

"#;

const ERROR_TSX: &str = r#"
const NotFoundPage = () => {
  return (
    <div>
      <h1>404 - Page Not Found</h1>
      <p>Sorry, the page you are looking for does not exist.</p>
    </div>
  );
};

export default NotFoundPage;
"#;

const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="stylesheet" href="index.css" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>NoApi Project</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="main.tsx"></script>
  </body>
</html>
"#;

const MAIN_TSX: &str = r#"
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { BrowserRouter } from "react-router-dom";
import React, { Fragment } from "react";
import { Routes, Route } from "react-router-dom";

type Module = { default: React.ComponentType };

const PRESERVED = import.meta.glob("/pages/(_app|404).tsx", {
  eager: true,
}) as Record<string, Module>;

const ROUTES = import.meta.glob("/pages/**/[a-z[]*.tsx", {
  eager: true,
}) as Record<string, Module>;

const preserved = Object.keys(PRESERVED).reduce((acc, file) => {
  const key = file.replace(/\/pages\/|\.tsx$/g, "");
  acc[key] = PRESERVED[file].default;
  return acc;
}, {} as Record<string, React.ComponentType>);

const routes = Object.keys(ROUTES).map((route) => {
  const path = route
    .replace(/\/pages|index|\.tsx$/g, "")
    .replace(/\[\.{3}.+\]/, "*")
    .replace(/\[(.+)\]/, ":$1");

  return { path, component: ROUTES[route].default };
});

const AppRoutes = () => {
  const App: React.ComponentType<{ children?: React.ReactNode }> = ({
    children,
  }) => {
    return <main>{children}</main>;
  };
  const NotFound = preserved["404"] || Fragment;

  return (
    <App>
      <Routes>
        {routes.map(({ path, component: Component }) => (
          <Route key={path} path={path} element={<Component />} />
        ))}
        <Route path="*" element={<NotFound />} />
      </Routes>
    </App>
  );
};

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <AppRoutes />
    </BrowserRouter>
  </StrictMode>
);
"#;

const VITE_ENV: &str = r#"
/// <reference types="vite/client" />
"#;

const GITIGNORE: &str = r#"
# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?

target

Cargo.lock

package-lock.json
"#;

const BUILD_RS: &str = r#"
use noapi_functions::build_functions::{
    build_frontend, rust_functions_to_axum_handlers, rust_to_typescript_functons,
};

fn main() {
    println!("cargo:rerun-if-changed=./src/functions.rs");
    rust_functions_to_axum_handlers("./src/functions.rs", "./src/handlers");
    println!("cargo:rerun-if-changed=./src/functions.rs");
    rust_to_typescript_functons("./src/functions.rs", "./functions.ts");
    println!("cargo:rerun-if-changed=frontend");
    build_frontend().unwrap()
}
"#;

const ESLINT_CONFIG: &str = r#"
import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";

export default tseslint.config(
  { ignores: ["dist"] },
  {
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
    plugins: {
      "react-hooks": reactHooks,
      "react-refresh": reactRefresh,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],
    },
  }
);
"#;

const README: &str = r#"
## NoApi Project

This template provides a minimal setup to get started with NoApi.
"#;

const TAILWIND_CONFIG: &str = r#"
module.exports = {
  content: ["./**/*.{ts,tsx}", "./frontend/**/*.{ts,tsx}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
"#;

const TSCONFIG_APP: &str = r#"
{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.app.tsbuildinfo",
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "isolatedModules": true,
    "moduleDetection": "force",
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true,
    "paths": {
      "@functions": ["./functions.ts"]
    }
  },
  "include": ["frontend"]
}
"#;

const TSCONFIG: &str = r#"
{
  "files": [],
  "references": [
    { "path": "./tsconfig.app.json" },
    { "path": "./tsconfig.node.json" }
  ]
}
"#;

const TSCONFIG_NODE: &str = r#"
{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.node.tsbuildinfo",
    "target": "ES2022",
    "lib": ["ES2023"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "isolatedModules": true,
    "moduleDetection": "force",
    "noEmit": true,

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true
  },
  "include": ["vite.config.ts"]
}
"#;

const VITE_CONFIG: &str = r#"
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  root: "frontend",
  build: {
    outDir: "..//src/static",
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      "@functions": path.resolve(__dirname, "functions.ts"),
    },
  },
});
"#;
