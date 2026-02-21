import vitest from "@vitest/eslint-plugin";
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
import globals from "globals";

import baseConfig from "../../eslint.config.js";
import { testFilePatterns, testFileSuffixes } from "../../eslint.helpers.js";

export default defineConfig(
	...baseConfig,

	{
		ignores: [
			"dist/*",
			"reports/*",
			"**/__generated__/*",
			"!**/__generated__/__mocks__/",
		],
	},

	{
		settings: {
			"import-x/resolver": {
				"eslint-import-resolver-typescript": {
					project: "./tconfig.json",
				},
			},
		},
	},

	{
		files: ["src/**/*.?(m|c)[tj]s?(x)"],
		languageOptions: {
			globals: { ...globals.browser },
		},
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
			// @ts-expect-error upstream types
			solid.configs["flat/recommended"],
			tailwindcss.configs["recommended-error"],
		],
		rules: {
			"no-console": "error",
			"@typescript-eslint/no-restricted-imports": [
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
			"import-x/no-unresolved": "off",
			"better-tailwindcss/enforce-consistent-line-wrapping": "off",
			"better-tailwindcss/enforce-shorthand-classes": "error",
			"better-tailwindcss/no-unknown-classes": "error",
		},
	},

	{
		files: testFilePatterns(),
		languageOptions: {
			globals: { ...globals.node },
		},
		rules: {
			"no-console": "off",
			"filenames/match-exported": [
				"error",
				{
					transforms: ["kebab"],
					remove: `\\.(${testFileSuffixes.join("|")})$`,
				},
			],
			"@typescript-eslint/no-explicit-any": "off",
			"@typescript-eslint/no-non-null-assertion": "off",
			"@typescript-eslint/no-unsafe-argument": "off",
			"@typescript-eslint/no-unsafe-assignment": "off",
			"@typescript-eslint/no-unsafe-call": "off",
			"@typescript-eslint/no-unsafe-member-access": "off",
			"@typescript-eslint/no-unsafe-return": "off",
			"@typescript-eslint/unbound-method": "off",
		},
	},

	{
		files: testFilePatterns({ root: "src" }),
		languageOptions: {
			globals: { ...globals.node, ...globals.browser },
		},
		extends: [
			vitest.configs.all,
			testingLibrary.configs["flat/dom"],
			jestDom.configs["flat/recommended"],
		],
		rules: {
			"vitest/no-disabled-tests": "error",
			"vitest/no-focused-tests": "error",
			"vitest/no-hooks": "off",
			"vitest/require-mock-type-parameters": "off",
		},
	},
);
