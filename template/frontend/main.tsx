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
  const App: React.ComponentType<{ children?: React.ReactNode }> =
    preserved["_app"] || Fragment;
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
