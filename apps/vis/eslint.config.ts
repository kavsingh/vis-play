// @ts-expect-error no types available
import jestDomPlugin from "eslint-plugin-jest-dom";
import solidPlugin from "eslint-plugin-solid";
// @ts-expect-error no types available
import tailwindPlugin from "eslint-plugin-tailwindcss";
// @ts-expect-error no types available
import testingPlugin from "eslint-plugin-testing-library";
import vitestPlugin from "eslint-plugin-vitest";
import globals from "globals";
import * as tsEslintPlugin from "typescript-eslint";

import baseConfig from "../../eslint.config";
import { resolveFrom, testFilePatterns } from "../../eslint.helpers";

const resolveLocal = resolveFrom(import.meta.url);

export default tsEslintPlugin.config(
	{
		extends: [...baseConfig],
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
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
		extends: [
			solidPlugin.configs["flat/recommended"],
			solidPlugin.configs["flat/typescript"],
			// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access
			...tailwindPlugin.configs["flat/recommended"],
		],
		rules: {
			"no-console": "error",
		},
	},

	{
		files: testFilePatterns({ root: "src" }),
		languageOptions: { globals: { ...globals.node, ...globals.browser } },
		// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
		extends: [
			vitestPlugin.configs.all,
			// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
			testingPlugin.configs.recommended,
			// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
			jestDomPlugin.configs["flat/recommended"],
		],
		rules: {
			"vitest/no-hooks": "off",
		},
	},
);
