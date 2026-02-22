import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "rolldown-vite";
import { checker } from "vite-plugin-checker";
import solid from "vite-plugin-solid";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import tsconfigPaths from "vite-tsconfig-paths";

import type { PluginOption } from "rolldown-vite";

function configChecker(mode: string) {
	if (mode !== "development") return undefined;

	return checker({
		overlay: { initialIsOpen: false },
		typescript: true,
	});
}

export default defineConfig(({ mode }) => ({
	base: "/vis-play/",
	build: { outDir: "dist", sourcemap: true },
	esbuild: {
		jsxImportSource: "solid-js",
		supported: { "top-level-await": true },
	},
	worker: { format: "es" },
	// oxlint-disable-next-line typescript/no-unsafe-type-assertion
	plugins: [
		tsconfigPaths(),
		solid(),
		tailwindcss(),
		// @ts-expect-error import resolution
		topLevelAwait(),
		// @ts-expect-error import resolution
		wasm(),
		configChecker(mode),
	] as PluginOption[],
}));
