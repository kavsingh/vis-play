import { onMount } from "solid-js";
import init, { vis, initThreadPool } from "vis-rs";

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
	try {
		await init();
		await initThreadPool(navigator.hardwareConcurrency);
		await vis();
	} catch (cause: unknown) {
		const error =
			cause instanceof Error ? cause : new Error("init error", { cause });

		if (
			error.message.startsWith(
				"Using exceptions for control flow, don't mind me. This isn't actually an error!",
			)
		) {
			logger.debug(error);
		} else {
			throw error;
		}
	}
}
