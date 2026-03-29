import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { checker } from "vite-plugin-checker";
import solid from "vite-plugin-solid";
import wasm from "vite-plugin-wasm";

import type { PluginOption } from "vite";

function configChecker(mode: string) {
	if (mode !== "development") return undefined;

	return checker({
		overlay: { initialIsOpen: false },
		oxlint: { lintCommand: "oxlint --type-aware --type-check" },
	});
}

export default defineConfig(({ mode }) => ({
	base: "/vis-play/",
	resolve: { tsconfigPaths: true },
	build: { outDir: "dist", sourcemap: true },
	oxc: {
		jsx: { importSource: "solid-js" },
		supported: { "top-level-await": true },
	},
	worker: { format: "es" },
	// oxlint-disable-next-line typescript/no-unsafe-type-assertion
	plugins: [
		solid(),
		tailwindcss(),
		// @ts-expect-error import resolution
		wasm(),
		configChecker(mode),
	] as PluginOption[],
}));
