/* eslint-disable no-console */

export const logger: Logger = {
	debug: import.meta.env.DEV ? console.debug : noop,
	info: import.meta.env.DEV ? console.info : noop,
	warn: console.warn,
	error: console.error,
};

export function scopedLogger(scope: string): Logger {
	const scopeTag = `[${scope}]`;

	return {
		debug: logger.debug.bind(scopeTag),
		info: logger.info.bind(scopeTag),
		warn: logger.warn.bind(scopeTag),
		error: logger.error.bind(scopeTag),
	};
}

export type Logger = {
	debug: typeof console.debug;
	info: typeof console.info;
	warn: typeof console.warn;
	error: typeof console.error;
};

function noop(..._: unknown[]) {
	// noop
}
