import { onCleanup } from "solid-js";

export default function Theme() {
	function handleQuery(ev: { matches: boolean }) {
		document.documentElement.classList.toggle("dark", ev.matches);
	}

	handleQuery({ matches: darkSchemeQuery.matches });

	darkSchemeQuery.addEventListener("change", handleQuery);

	onCleanup(() => {
		darkSchemeQuery.removeEventListener("change", handleQuery);
	});

	return null;
}

const darkSchemeQuery = window.matchMedia("(prefers-color-scheme: dark)");
