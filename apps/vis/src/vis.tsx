import { onMount } from "solid-js";

import { scopedLogger } from "#logger";

const logger = scopedLogger("<Vis />");

export default function Vis() {
	onMount(() => {
		void startVis();
	});

	return (
		<div class="grid size-full place-items-center bg-black">
			<canvas
				id="vis-rs"
				width={1024}
				height={768}
				class="focus:outline-none"
			/>
		</div>
	);
}

async function startVis() {
	const { vis } = await import("vis-rs");

	logger.info("starting vis");

	try {
		vis();
	} catch (cause) {
		if (/isn't actually an error/i.test(String(cause))) {
			logger.debug(cause);
		} else {
			logger.error("failed to start", cause);
		}
	}
}
