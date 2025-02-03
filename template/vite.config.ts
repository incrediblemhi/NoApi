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
