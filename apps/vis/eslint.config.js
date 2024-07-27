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
import tsEslint from "typescript-eslint";

import baseConfig from "../../eslint.config.js";
import {
	testFilePatterns,
	getImportOrderConfig,
} from "../../eslint.helpers.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const project = path.resolve(__dirname, "tsconfig.json");

export default tsEslint.config(
	{
		extends: [...baseConfig],
		languageOptions: { parserOptions: { project } },
	},
	{
		files: ["src/**/*"],
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
		extends: [
			solidPlugin.configs["flat/recommended"],
			solidPlugin.configs["flat/typescript"],
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
		extends: [
			vitestPlugin.configs.all,
			testingPlugin.configs.recommended,
			jestDomPlugin.configs["flat/recommended"],
		],
		rules: {
			"vitest/no-hooks": "off",
		},
	},
);
