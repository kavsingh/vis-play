/// <reference types="vite/client" />

import { render } from "solid-js/web";

import "./index.css";
import App from "./app.tsx";

const appRoot = document.getElementById("app-root");

if (!appRoot) throw new Error("#app-root not found");

render(() => <App />, appRoot);
