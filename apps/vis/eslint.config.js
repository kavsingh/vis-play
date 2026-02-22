import jestDom from "eslint-plugin-jest-dom";
import solid from "eslint-plugin-solid";
import testingLibrary from "eslint-plugin-testing-library";
import { defineConfig } from "eslint/config";
import { configs as tsEslint } from "typescript-eslint";

export default defineConfig(
	{
		files: ["src/**/*.{ts,tsx}"],
		extends: [
			tsEslint.base,
			// @ts-expect-error upstream types
			solid.configs["flat/recommended"],
		],
	},

	{
		files: ["src/**/*.test.{ts,tsx}"],
		extends: [
			testingLibrary.configs["flat/dom"],
			jestDom.configs["flat/recommended"],
		],
	},
);
