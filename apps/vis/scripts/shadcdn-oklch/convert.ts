import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

// @ts-expect-error no types available
import cssjson from "cssjson";
import { parse, oklch } from "culori";
import { format as prettierFormat } from "prettier";

import prettierConfig from "../../../../prettier.config.js";

const dirname = fileURLToPath(new URL(".", import.meta.url));

async function themeToOklch() {
	const contents = await fs.readFile(
		path.resolve(dirname, "./source-theme.css"),
		"utf-8",
	);

	/* eslint-disable @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-argument */
	const asJson = cssjson.toJSON(contents).children["@layer base"];
	const rootStyles = convert(asJson.children[":root"].attributes);
	const darkStyles = convert(asJson.children[".dark"].attributes);
	// eslint-enable

	const formatted = await format(rootStyles, darkStyles);

	return fs.writeFile(path.resolve(dirname, "../../src/theme.css"), formatted);
}

function format(
	rootStyles: Record<string, string>,
	darkStyles: Record<string, string>,
) {
	const content = `
		/* https://ui.shadcn.com/themes, but converted to oklch color space */
		/* generated, do not modify directly */

		@layer base {
			:root {
				${Object.entries(rootStyles)
					.map(([k, v]) => `${k}: ${v};`)
					.join("\n")}
			}
			
			.dark {
				${Object.entries(darkStyles)
					.map(([k, v]) => `${k}: ${v};`)
					.join("\n")}
			}
		}
	`;

	return prettierFormat(content, { ...prettierConfig, parser: "css" });
}

function convert(colorMap: Record<string, string>) {
	const converted: Record<string, string> = {};

	for (const [key, value] of Object.entries(colorMap)) {
		converted[key] = convertColor(value);
	}

	return converted;
}

function convertColor(maybeHsl: string) {
	try {
		const parsed = parse(`hsl(${maybeHsl})`);

		if (!parsed) throw new Error(`failed to parse ${maybeHsl}`);

		const converted = oklch(parsed);

		return `${converted.l} ${converted.c} ${converted.h ?? 0}`;
	} catch (_) {
		console.warn(`could not parse ${maybeHsl}, skipping...`);

		return maybeHsl;
	}
}

void themeToOklch();
