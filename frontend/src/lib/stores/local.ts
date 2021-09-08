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

export const theme = createLocalStore("theme", "light");
browser && theme.useLocalStorage();

export const achieve = createLocalStore("achieve", { isAchievementShown: false });

let initalSubscribe = true;
browser &&
    theme.subscribe((current) => {
        if (current === "dark") {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }
        if (current === "dark") {
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
