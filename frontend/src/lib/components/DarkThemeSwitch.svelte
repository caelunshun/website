<script lang="ts">
    import { theme } from "$lib/stores/local";

    import ClockOutline from "$lib/assets/minecraft/clock_outline.svg?component";

    let extraClazz: string;
    export { extraClazz as class };

    function toggleDarkMode() {
        $theme = $theme === "dark" ? "light" : "dark";
    }

    function darkModeSwitchKeyPressHandler(event: KeyboardEvent) {
        if (event.code === "Space") toggleDarkMode();
    }
</script>

<div
    class="w-16 h-8 flex items-center dark:bg-gray-900 bg-gray-300 rounded-full cursor-pointer {extraClazz}"
    role="switch"
    aria-checked={$theme === "dark"}
    aria-label="Toggle Dark Mode"
    on:keyup={darkModeSwitchKeyPressHandler}
    on:click={toggleDarkMode}
    tabindex={0}
>
    <div
        class="w-8 h-8 rounded-full shadow-md transform my-auto dark:translate-x-8 duration-300 ease-in-out flex items-center justify-center"
    >
        <div class="relative w-8 h-8">
            <ClockOutline class="absolute" />
            <div class="mc-clock-after w-8 h-8 dark:clockdark" />
        </div>
    </div>
</div>
