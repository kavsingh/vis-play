/// <reference types="vitest" />

import path from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig } from "vite";
import { checker as checkerPlugin } from "vite-plugin-checker";
import solidPlugin from "vite-plugin-solid";
import topLevelAwaitPlugin from "vite-plugin-top-level-await";
import wasmPlugin from "vite-plugin-wasm";
import tsconfigPathsPlugin from "vite-tsconfig-paths";

import type { PluginOption } from "vite";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig(({ mode }) => ({
	build: { sourcemap: true },
	plugins: [
		tsconfigPathsPlugin({
			projects: [path.resolve(__dirname, "tsconfig.json")],
		}),
		topLevelAwaitPlugin(),
		solidPlugin(),
		wasmPlugin(),
		checker(mode),
	] as PluginOption[],
	test: {
		include: ["src/**/*.{test,spec}.?([mc])[tj]s?(x)"],
		environment: "jsdom",
		setupFiles: ["./vitest.setup.ts"],
		clearMocks: true,
		testTransformMode: { web: ["/.[jt]sx?$/"] },
		server: { deps: { inline: [/@solidjs/] } },
	},
}));

function checker(mode: string) {
	if (mode !== "development") return undefined;

	return checkerPlugin({
		overlay: { initialIsOpen: false },
		typescript: true,
	});
}
