import { onMount } from "solid-js";

export default function Nannou() {
	let containerEl: HTMLDivElement | null = null;

	async function startAndContain() {
		if (!containerEl) return;

		await startNannou();

		const nannouEl = document.querySelector("[alt='nannou vis']");

		if (!(nannouEl instanceof HTMLCanvasElement)) return;

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

async function startNannou() {
	const { main_web: nannou } = await import("vis-rs");

	await nannou();
}
