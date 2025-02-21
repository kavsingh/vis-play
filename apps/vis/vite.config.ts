import { defineConfig } from "vite";
import { checker as checkerPlugin } from "vite-plugin-checker";
import solidPlugin from "vite-plugin-solid";
import topLevelAwaitPlugin from "vite-plugin-top-level-await";
import wasmPlugin from "vite-plugin-wasm";
import tsconfigPathsPlugin from "vite-tsconfig-paths";

import type { PluginOption } from "vite";

export default defineConfig(({ mode }) => ({
	build: { sourcemap: true },
	esbuild: { supported: { "top-level-await": true } },
	plugins: [
		tsconfigPathsPlugin(),
		solidPlugin(),
		// @ts-expect-error import resolution
		topLevelAwaitPlugin(),
		// @ts-expect-error import resolution
		wasmPlugin(),
		checker(mode),
	] as PluginOption[],
}));

function checker(mode: string) {
	if (mode !== "development") return undefined;

	return checkerPlugin({
		overlay: { initialIsOpen: false },
		typescript: true,
	});
}
