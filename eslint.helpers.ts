import path from "node:path";
import { fileURLToPath } from "node:url";

export function resolveFrom(fileUrl: string | URL) {
	const dirname = path.dirname(fileURLToPath(fileUrl));

	return function resolve(...parts: string[]) {
		return path.resolve(dirname, ...parts);
	};
}

export const testFileSuffixes = ["test", "spec", "mock"];

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
