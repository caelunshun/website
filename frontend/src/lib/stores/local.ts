import { writable } from "svelte/store";
import type { Writable } from "svelte/store";
import { browser } from "$app/env";

interface LocalStore<T> extends Writable<T> {
    useLocalStorage: () => void;
}

const createLocalStore = <T>(key: string, startValue: T): LocalStore<T> => {
    const { subscribe, set, update } = writable(startValue);
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
        }
    };
};

let darkM = true;
if (browser) {
    darkM = window.matchMedia("(prefers-color-scheme: dark)").matches;
}
export const preferences = createLocalStore("pref", { dark: darkM });
browser && preferences.useLocalStorage();

export const achieve = createLocalStore("achieve", { isAchievementShown: false });

let initalSubscribe = true;
browser &&
    preferences.subscribe((current) => {
        if (initalSubscribe) {
            initalSubscribe = false;
            return;
        }
        if (current.dark === true) {
            const wasAchievementShown = localStorage.getItem("wasAchievementShown");
            if (!wasAchievementShown) {
                achieve.set({ isAchievementShown: true });
                setTimeout(() => {
                    achieve.set({ isAchievementShown: false });
                }, 5000);
                localStorage.setItem("wasAchievementShown", "yes");
            }
        }
    });
