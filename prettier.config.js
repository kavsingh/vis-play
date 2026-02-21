/** @type {import('prettier').Config} */
export default {
	useTabs: true,
	printWidth: 80,
	overrides: [
		{
			files: ["*.jsonc", "tsconfig.json", "tsconfig.*.json", ".zed/*.json"],
			options: { parser: "jsonc", trailingComma: "none" },
		},
	],
};
