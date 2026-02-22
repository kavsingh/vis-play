import { onCleanup, onMount } from "solid-js";

function handleQuery(ev: { matches: boolean }) {
	document.documentElement.classList.toggle("dark", ev.matches);
}

export function Theme() {
	const darkSchemeQuery = window.matchMedia("(prefers-color-scheme: dark)");

	onMount(() => {
		handleQuery({ matches: darkSchemeQuery.matches });
		darkSchemeQuery.addEventListener("change", handleQuery);
	});

	onCleanup(() => {
		darkSchemeQuery.removeEventListener("change", handleQuery);
	});

	return null;
}
