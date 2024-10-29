import path from "node:path";
import { fileURLToPath } from "node:url";

/** @param {string} fileUrl */
export function resolveFrom(fileUrl) {
	const dirname = path.dirname(fileURLToPath(fileUrl));

	return function resolve(/** @type {string[]} */ ...parts) {
		return path.resolve(dirname, ...parts);
	};
}

export const testFileSuffixes = /** @type {const} */ (["test", "spec", "mock"]);

export function testFilePatterns({
	root = "",
	extensions = "?([mc])[tj]s?(x)",
} = {}) {
	return [
		`*.{${testFileSuffixes.join(",")}}`,
		"__{test,tests,mocks,fixtures}__/**/*",
		"__{test,mock,fixture}-*__/**/*",
	].map((pattern) => path.join(root, `**/${pattern}.${extensions}`));
}
