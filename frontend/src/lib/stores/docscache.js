// Not a svelte store as it isn't reactive

const cache = {};

export function set(page, content) {
  cache[page] = content;
}

export function has(page) {
  return cache[page] !== undefined;
}

export function get(page) {
  return cache[page];
}
