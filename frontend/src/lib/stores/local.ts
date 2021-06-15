import { writable, Writable } from "svelte/store";
import { browser } from "$app/env";

import type {Topic} from "$lib/types";

interface LocalStore<T> extends Writable<T> {
  useLocalStorage: () => void;
}

const createLocalStore = <T>(key: string, startValue: T): LocalStore<T>  => {
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

export const token = createLocalStore("token", {secret: undefined});

export const preferences = (() => {
  let darkM = false;
  if (browser) {
    darkM = window.matchMedia("(prefers-color-scheme: dark)").matches;
  }
  const a = createLocalStore("pref", { dark: darkM });
  // a.useLocalStorage();
  return a;
})();

export const docheadings = createLocalStore<Topic[]>("docheadings", []);
