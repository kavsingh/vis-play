import solid from "eslint-plugin-solid";
import { defineConfig } from "eslint/config";
import { configs as tsEslint } from "typescript-eslint";

export default defineConfig({
	files: ["src/**/*.{ts,tsx}"],
	extends: [
		tsEslint.base,
		// @ts-expect-error upstream types
		solid.configs["flat/typescript"],
	],
});
