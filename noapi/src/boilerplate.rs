use std::{fs, path::Path};

use crate::config::{Config, JsRuntime};

pub fn generate_boilerplate(project_name: &str, config: Config) -> std::io::Result<()> {
    let package_json: &str = &format!(
        r#"{{
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
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8.1"
listenfd = "1.0.2"
tokio = {{version = "1.43.0", features = ["full"]}}
tower = "0.5.2"
tower-http = {{ version = "0.6.2", features = ["fs", "trace"] }}
regex = "1.11.1"
noapi-functions = "0.1.0"
serde = {{version = "1.0.217", features = ["derive"]}}
serde_json = "1.0.138"
tower-livereload = "0.9.6"
tracing = "0.1.41"
tracing-subscriber = {{version = "0.3.19", features = ["env-filter"]}}

[build-dependencies]
noapi-functions = "0.1.41"
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
    fs::write(project_path.join("frontend").join("index.css"), INDEX_CSS)?;
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
    if config.js_runtime == JsRuntime::Deno {
        fs::write(project_path.join("deno.json"), DENO_JSON)?;
        fs::write(project_path.join("vite.config.ts"), DENO_VITE_CONFIG)?;
    } else {
        fs::write(project_path.join("eslint.config.js"), ESLINT_CONFIG)?;
        fs::write(project_path.join("package.json"), package_json)?;
        fs::write(project_path.join("tsconfig.json"), TSCONFIG)?;
        fs::write(project_path.join("tsconfig.node.json"), TSCONFIG_NODE)?;
        fs::write(project_path.join("vite.config.ts"), VITE_CONFIG)?;
    }
    fs::write(project_path.join("tailwind.config.js"), TAILWIND_CONFIG)?;
    fs::write(project_path.join("Cargo.toml"), cargo_toml)?;
    fs::write(project_path.join(".gitignore"), GITIGNORE)?;
    fs::write(project_path.join("functions.ts"), "")?;
    fs::write(project_path.join("NoApi.toml"), config.to_string())?;
    fs::write(project_path.join("build.rs"), BUILD_RS)?;

    println!(
        "New NoApi project generated at /{}\n",
        project_path.to_str().unwrap()
    );
    println!(
        "Now run:\n- cd {}\n- noapi install\n- noapi runserver",
        project_name
    );

    Ok(())
}

const MAIN_RS: &str = r#"pub mod functions;
pub mod handlers;

use handlers::create_router;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut app = create_router();

    app = app.layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
"#;

const FUNCTIONS_RS: &str = r#"#[derive(serde::Serialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

pub fn create_user(email: String, password: String, _username: String) -> User {
    User { email, password }
}
"#;

const INDEX_TSX: &str = r#"import { useRef } from "react";
import { create_user } from "@functions";

const App = () => {
  const usernameRef = useRef<HTMLInputElement>(null);
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const username = usernameRef.current?.value;
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;

    if (username && email && password) {
      create_user(email, password, username)
        .then((res) => {
          console.log("User created:", res);
        })
        .catch((err) => {
          console.error("Error creating user:", err);
        });
    }
  };

  return (
    <main className=" flex flex-row max-sm:flex-col font-sans w-full h-screen">
      <div className="flex flex-col w-1/2 max-sm:w-full text-center items-center justify-center bg-black text-white">
        <h1 className="text-xl ">What is going on here??</h1>
        <br />
        <p className="w-full max-w-sm min-w-[200px]">
          When you click on the "create user" button, the typescript fuction
          "create_user()" calls a corresponding rust function also called
          "create_user()" found in the src/functions.rs and passes the
          parameters and the response all in their correct types. <br />
          What this means is that you can create rust functions in the
          src/functions.rs file and directly import them into your typescript
          app, all without doing any extra work of creating APIs or validating
          its data and return types. <br />
          It just works.
        </p>
      </div>
      <div className="flex flex-col items-center justify-center w-1/2 max-sm:w-full">
        <form
          onSubmit={handleSubmit}
          className="flex flex-col items-center justify-center w-full max-w-sm min-w-[200px] space-y-3"
        >
          <h6>Create A User</h6>
          <input
            type="text"
            name="username"
            ref={usernameRef}
            placeholder="Username"
            required
            className="w-full bg-transparent text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
          />
          <input
            type="email"
            name="email"
            ref={emailRef}
            placeholder="Email"
            required
            className="w-full bg-transparent text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
          />
          <input
            type="password"
            name="password"
            ref={passwordRef}
            placeholder="Password"
            required
            className="w-full bg-transparent text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
          />
          <button type="submit">Create User</button>
        </form>
      </div>
    </main>
  );
};

export default App;
"#;

const ERROR_TSX: &str = r#"const NotFoundPage = () => {
  return (
    <div>
      <h1>404 - Page Not Found</h1>
      <p>Sorry, the page you are looking for does not exist.</p>
    </div>
  );
};

export default NotFoundPage;
"#;

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="stylesheet" href="index.css" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>NoApi Project</title>
  </head>
  <body>
    <div id="root" class="w-full min-h-[100vh]"></div>
    <script type="module" src="main.tsx"></script>
  </body>
</html>
"#;

const INDEX_CSS: &str = r#"@import "tailwindcss";"#;

const MAIN_TSX: &str = r#"import { StrictMode } from "react";
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

const VITE_ENV: &str = r#"/// <reference types="vite/client" />"#;

const GITIGNORE: &str = r#"# Logs
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

Bun.lock

deno.lock
"#;

const BUILD_RS: &str = r#"use noapi_functions::build_functions::{
    build_frontend, rust_functions_to_axum_handlers, rust_to_typescript_functons,
};

fn main() {
    println!("cargo:rerun-if-changed=./src/functions.rs");
    rust_functions_to_axum_handlers("./src/functions.rs", "./src/handlers");
    println!("cargo:rerun-if-changed=./src/functions.rs");
    rust_to_typescript_functons(
        "./src/functions.rs",
        "./functions.ts",
        env!("CARGO_PKG_NAME"),
    );
    println!("cargo:rerun-if-changed=frontend");
    build_frontend().unwrap()
}
"#;

const ESLINT_CONFIG: &str = r#"import js from "@eslint/js";
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

const TAILWIND_CONFIG: &str = r#"module.exports = {
  content: ["./**/*.{ts,tsx}", "./frontend/**/*.{ts,tsx}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
"#;

const TSCONFIG: &str = r#"{
  "files": [],
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
  "include": ["frontend"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
"#;

const TSCONFIG_NODE: &str = r#"{
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
    "emitDeclarationOnly": true,
    "composite": true,

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

const VITE_CONFIG: &str = r#"import { defineConfig } from "vite";
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

const DENO_VITE_CONFIG: &str = r#"import { defineConfig } from "vite";
import deno from "@deno/vite-plugin";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";

// https://vite.dev/config/
export default defineConfig({
  // @ts-expect-error Unable to infer type at the moment
  plugins: [deno(), react(), tailwindcss()],
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

const DENO_JSON: &str = r#"{
  "tasks": {
    "dev": "deno run -A --node-modules-dir npm:vite",
    "build": "deno run -A --node-modules-dir npm:vite build",
    "preview": "deno run -A --node-modules-dir npm:vite preview",
    "serve": "deno run --allow-net --allow-read jsr:@std/http@1/file-server dist/"
  },
  "compilerOptions": {
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "jsx": "react-jsx",
    "jsxImportSource": "react",
    "jsxImportSourceTypes": "@types/react"
  },
  "imports": {
    "@deno/vite-plugin": "npm:@deno/vite-plugin@^1.0.0",
    "@types/react": "npm:@types/react@^18.3.12",
    "@types/react-dom": "npm:@types/react-dom@^18.3.1",
    "@vitejs/plugin-react": "npm:@vitejs/plugin-react@^4.3.4",
    "react": "npm:react@^18.3.1",
    "react-dom": "npm:react-dom@^18.3.1",
    "vite": "npm:vite@^6.0.1",
    "@tailwindcss/vite": "npm:@tailwindcss/vite@^4.0.0",
    "axios": "npm:axios@^1.7.9",
    "react-router-dom": "npm:react-router-dom@^7.1.5",
    "globals": "npm:globals@^15.14.0",
    "tailwindcss": "npm:tailwindcss@^4.0.0",
    "@types/node": "npm:@types/node@^22.13.0"
  }
}
"#;
