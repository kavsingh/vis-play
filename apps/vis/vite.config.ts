import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { checker } from "vite-plugin-checker";
import solid from "vite-plugin-solid";
import wasm from "vite-plugin-wasm";

export default defineConfig(({ mode }) => ({
	base: "/vis-play/",
	resolve: { tsconfigPaths: true },
	build: { outDir: "dist", sourcemap: true },
	oxc: {
		jsx: { importSource: "solid-js" },
		supported: { "top-level-await": true },
	},
	worker: { format: "es" },
	plugins: [
		solid(),
		tailwindcss(),
		// @ts-expect-error import resolution
		wasm(),
		mode === "development"
			? checker({ oxlint: true, overlay: { initialIsOpen: false } })
			: undefined,
	],
}));
