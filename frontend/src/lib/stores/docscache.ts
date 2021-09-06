// Not a svelte store as it isn't reactive

const cache = {};

export function set(page: string, content: string): void {
    cache[page] = content;
}

export function has(page: string): boolean {
    return cache[page] !== undefined;
}

export function get(page: string): string {
    return cache[page];
}
