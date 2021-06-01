import { writable } from "svelte/store";
import { browser } from "$app/env";

const createLocalStore = (key, startValue) => {
  const {
    subscribe,
    set,
    update,
  } = writable(startValue);

  return {
    subscribe,
    set,
    update,
    useLocalStorage: () => {
      const json = localStorage.getItem(key);
      if (json) {
        set(JSON.parse(json));
      }

      subscribe((current) => {
        localStorage.setItem(key, JSON.stringify(current));
      });
    },
  };
};

export const token = createLocalStore("token", {});

export const preferences = (() => {
  let darkM = false;
  if (browser) {
    darkM = window.matchMedia("(prefers-color-scheme: dark)").matches;
  }
  const a = createLocalStore("pref", { dark: darkM });
  // a.useLocalStorage();
  return a;
})();
