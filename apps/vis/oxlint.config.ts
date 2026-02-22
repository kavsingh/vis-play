import { defineConfig } from "oxlint";

import base from "../../oxlint.config.ts";

export default defineConfig({
	ignorePatterns: ["dist/*", "reports/*"],
	extends: [base],
	settings: {
		vitest: { typecheck: true },
	},
	overrides: [
		{
			files: ["./src/**/*.{ts,tsx}"],
			plugins: ["import"],
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
