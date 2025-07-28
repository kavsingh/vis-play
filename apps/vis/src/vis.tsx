import { onCleanup, onMount } from "solid-js";

import { scopedLogger } from "#logger";

const logger = scopedLogger("<Vis />");

export default function Vis() {
	let containerRef: HTMLDivElement | undefined;
	let canvasRef: HTMLCanvasElement | undefined;

	function updateSizes() {
		if (!(containerRef && canvasRef)) return;

		canvasRef.style.width = `${containerRef.offsetWidth}px`;
		canvasRef.style.height = `${containerRef.offsetHeight}px`;
	}

	onMount(() => {
		startVis()
			.then(() => {
				window.addEventListener("resize", updateSizes);
				updateSizes();
			})
			.catch(logger.error);
	});

	onCleanup(() => {
		window.removeEventListener("resize", updateSizes);
	});

	return (
		<div
			class="grid size-full place-items-center bg-black"
			ref={(el) => (containerRef = el)}
		>
			<canvas
				id="vis-rs"
				class="focus:outline-none"
				ref={(el) => (canvasRef = el)}
			/>
		</div>
	);
}

async function startVis() {
	const { vis } = await import("vis-rs");

	logger.info("starting vis");

	try {
		vis(import.meta.env.DEV ? 600 : 6_000);
	} catch (cause) {
		if (/isn't actually an error/i.test(String(cause))) {
			logger.debug(cause);
		} else {
			logger.error("failed to start", cause);
		}
	}
}
