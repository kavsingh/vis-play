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
	const { default: init, initThreadPool, vis } = await import("vis-rs");

	try {
		await init();
		await initThreadPool(navigator.hardwareConcurrency);
		await vis();
	} catch (cause: unknown) {
		const error =
			cause instanceof Error ? cause : new Error("startVis error", { cause });
		const isDeliberate =
			/Using exceptions for control flow, don't mind me. This isn't actually an error!/i.test(
				error.message,
			);

		if (isDeliberate) logger.debug(error);
		else throw error;
	}
}
