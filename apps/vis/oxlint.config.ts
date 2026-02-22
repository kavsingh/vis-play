import path from "node:path";

import tailwindcss from "eslint-plugin-better-tailwindcss";
import { getDefaultSelectors } from "eslint-plugin-better-tailwindcss/defaults";
import {
	MatcherType,
	SelectorKind,
} from "eslint-plugin-better-tailwindcss/types";
import { defineConfig } from "oxlint";

import base from "../../oxlint.config.ts";

export default defineConfig({
	ignorePatterns: ["dist/*", "reports/*"],
	extends: [base],
	settings: {
		vitest: { typecheck: true },
		"better-tailwindcss": {
			entryPoint: path.resolve(import.meta.dirname, "./src/app.css"),
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
	overrides: [
		{
			files: ["./src/**/*.{ts,tsx}"],
			plugins: ["import"],
			jsPlugins: ["eslint-plugin-better-tailwindcss"],
			rules: {
				"eslint/no-console": "error",
				"eslint/no-restricted-imports": [
					"error",
					{
						paths: [
							{
								name: "tailwind-merge",
								message: "please import helpers from #src/style",
							},
							{
								name: "tailwind-variants",
								message: "please import helpers from #src/style",
							},
						],
					},
				],

				"import/no-nodejs-modules": "error",
				"import/no-unassigned-import": ["error", { allow: ["**/*.css"] }],

				...tailwindcss.configs["recommended-error"].rules,
				"better-tailwindcss/enforce-consistent-line-wrapping": "off",
				"better-tailwindcss/enforce-shorthand-classes": "error",
			},
		},

		{
			files: ["./src/**/*.test.{ts,tsx}"],
			plugins: ["vitest"],
			rules: {
				"eslint/no-console": "off",

				"vitest/no-disabled-tests": "error",
				"vitest/no-focused-tests": "error",
				"vitest/no-import-node-test": "error",
			},
		},
	],
});
