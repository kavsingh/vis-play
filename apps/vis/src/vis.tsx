import { onMount } from "solid-js";

export default function Vis() {
	let containerEl: HTMLDivElement | null = null;

	async function startAndContain() {
		if (!containerEl) return;

		await startVis();

		const nannouEl = document.querySelector("[alt='vis-rs']");

		if (!(nannouEl instanceof HTMLCanvasElement)) {
			throw new Error("expected a canvas element with alt=vis-rs");
		}

		containerEl.appendChild(nannouEl);
	}

	onMount(() => {
		void startAndContain();
	});

	return (
		<div
			class="grid size-full place-items-center bg-black"
			ref={(el) => (containerEl = el)}
		/>
	);
}

async function startVis() {
	const { main_web: main } = await import("vis-rs");

	await main();
}
