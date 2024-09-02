import path from "node:path";
import { fileURLToPath } from "node:url";

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
import { testFilePatterns, getImportOrderConfig } from "../../eslint.helpers";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const project = path.resolve(__dirname, "tsconfig.json");

export default tsEslintPlugin.config(
	{
		extends: [...baseConfig],
		languageOptions: { parserOptions: { projectService: true } },
	},
	{
		files: ["src/**/*.?([mc])[tj]s?(x)"],
		languageOptions: { globals: { ...globals.browser } },
		settings: {
			"import-x/resolver": {
				"eslint-import-resolver-typescript": { project },
			},
			"tailwindcss": {
				config: path.resolve(__dirname, "tailwind.config.ts"),
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
			"import-x/order": getImportOrderConfig(project),
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
