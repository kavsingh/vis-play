import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { checker as checker } from "vite-plugin-checker";
import solid from "vite-plugin-solid";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import tsconfigPaths from "vite-tsconfig-paths";

import type { PluginOption } from "vite";

export default defineConfig(({ mode }) => ({
	base: "/vis-play/",
	build: { outDir: "dist", sourcemap: true },
	esbuild: { supported: { "top-level-await": true } },
	worker: { format: "es" },
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
	server: {
		headers: {
			"Cross-Origin-Opener-Policy": "same-origin",
			"Cross-Origin-Embedder-Policy": "require-corp",
		},
	},
	preview: {
		headers: {
			"Cross-Origin-Opener-Policy": "same-origin",
			"Cross-Origin-Embedder-Policy": "require-corp",
		},
	},
}));

function configChecker(mode: string) {
	if (mode !== "development") return undefined;

	return checker({
		overlay: { initialIsOpen: false },
		typescript: true,
	});
}
