import { fixupPluginRules } from "@eslint/compat";
// @ts-expect-error no types available
import jestDom from "eslint-plugin-jest-dom";
import solid from "eslint-plugin-solid";
import tailwindcss from "eslint-plugin-tailwindcss";
// @ts-expect-error no types available
import testingLibrary from "eslint-plugin-testing-library";
import vitest from "eslint-plugin-vitest";
import globals from "globals";
import * as tsEslint from "typescript-eslint";

import baseConfig from "../../eslint.config.js";
import { resolveFrom, testFilePatterns } from "../../eslint.helpers.js";

const resolveLocal = resolveFrom(import.meta.url);

export default tsEslint.config(
	...baseConfig,

	{
		ignores: ["dist/*", "coverage/*"],
	},

	{
		files: ["src/**/*.?([mc])[tj]s?(x)"],
		languageOptions: { globals: { ...globals.browser } },
		settings: {
			"import-x/resolver": {
				"eslint-import-resolver-typescript": {
					project: resolveLocal("tsconfig.json"),
				},
			},
			"tailwindcss": {
				config: resolveLocal("tailwind.config.ts"),
				callees: ["tv", "classList"],
			},
		},

		extends: [
			solid.configs["flat/recommended"],
			solid.configs["flat/typescript"],
			...tailwindcss.configs["flat/recommended"],
		],
		rules: {
			"no-console": "error",
		},
	},

	{
		files: testFilePatterns({ root: "src" }),
		languageOptions: {
			globals: { ...globals.node, ...globals.browser },
		},
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
		extends: [
			vitest.configs.all,
			// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
			jestDom.configs["flat/recommended"],
		],
		plugins: {
			"testing-library": fixupPluginRules({
				// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access
				rules: testingLibrary.rules,
			}),
		},
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
		rules: {
			// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
			...testingLibrary.configs["flat/dom"].rules,
			"vitest/no-hooks": "off",
		},
	},
);
