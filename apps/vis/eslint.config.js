import tailwindcss from "eslint-plugin-better-tailwindcss";
import { getDefaultSelectors } from "eslint-plugin-better-tailwindcss/defaults";
import {
	MatcherType,
	SelectorKind,
} from "eslint-plugin-better-tailwindcss/types";
import jestDom from "eslint-plugin-jest-dom";
import solid from "eslint-plugin-solid";
import testingLibrary from "eslint-plugin-testing-library";
import { defineConfig } from "eslint/config";
import { configs as tsEslint } from "typescript-eslint";

export default defineConfig(
	{
		files: ["src/**/*.{ts,tsx}"],
		settings: {
			"better-tailwindcss": {
				entryPoint: "src/app.css",
				selectors: [
					...getDefaultSelectors(),
					...["tj", "tm"].map((name) => ({
						name,
						kind: SelectorKind.Callee,
						match: [{ type: MatcherType.String }],
					})),
				],
			},
		},
		extends: [
			tsEslint.base,
			// @ts-expect-error upstream types
			solid.configs["flat/recommended"],
			tailwindcss.configs["recommended-error"],
		],
		rules: {
			"better-tailwindcss/enforce-consistent-line-wrapping": "off",
			"better-tailwindcss/enforce-shorthand-classes": "error",
			"better-tailwindcss/no-unknown-classes": "error",
		},
	},

	{
		files: ["src/**/*.test.{ts,tsx}"],
		extends: [
			testingLibrary.configs["flat/dom"],
			jestDom.configs["flat/recommended"],
		],
	},
);
