<script lang="ts">
    import "../global.css";
    import { achieve } from "$lib/stores/local";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import Logo from "$lib/assets/chicken2.svg?component";
    import GithubMark from "$lib/assets/github-mark.svg?component";
    import DarkThemeSwitch from "$lib/components/DarkThemeSwitch.svelte";
    import { MenuIcon } from "svelte-feather-icons";
    import Achievement from "$lib/components/Achievement.svelte";
    import Headroom from "$lib/components/Headroom.svelte";

    let isDropdownShown: boolean = false;

    onMount(() => {
        const anchorclicklistener = () => {
            if (isDropdownShown) {
                isDropdownShown = false;
            }
        };
        document
            .querySelectorAll('header a[href^="/"]')
            .forEach((anchorelem) =>
                (anchorelem as HTMLAnchorElement).addEventListener(
                    "click",
                    anchorclicklistener,
                    false
                )
            );
    });

    let scrollY: number;
</script>

<svelte:window bind:scrollY />

<Headroom offset={100}>
    <header
        class="flex bg-green-600 dark:bg-nether-netherrack dark:netherrack text-white transition-colors duration-500"
        role="heading"
        aria-level={1}
    >
        <nav
            class="flex flex-1 flex-col md:flex-row items-center md:h-24 py-4 px-4 container mx-auto relative {isDropdownShown
                ? 'max-h-96'
                : 'max-h-20 md:max-h-24'}"
            style="transition: max-height .5s;"
            role="navigation"
            aria-label="Main"
        >
            <div class="flex w-full justify-between">
                <a href="/" class="flex items-center">
                    <Logo class="h-8 sm:h-12" />
                    <div class="ml-3 text-4xl block"><span>Feather</span></div></a
                >
                <div
                    class="flex md:hidden z-40 text-white items-center"
                    on:click={() => {
                        isDropdownShown = !isDropdownShown;
                    }}
                >
                    <MenuIcon class="text-white h-10 w-10" />
                </div>
            </div>
            <ul
                class="md:flex w-full md:w-auto items-center md:space-x-6 space-y-2 md:space-y-0 mt-4 md:mt-0 text-normal sm:text-xl font-bold {isDropdownShown
                    ? ''
                    : 'hidden md:block'}"
            >
                <li class="hover:underline text-xl p-2"><a href="/download">Download</a></li>
                <li class="hover:underline text-xl p-2"><a href="/faq">FAQ</a></li>
                <hr class="md:hidden" />
                <li class="flex justify-end">
                    <a
                        href="https://github.com/feather-rs/feather"
                        class="flex items-center bg-github p-1.5 rounded-full"
                        ><GithubMark class="h-8 md:h-10 fill-current text-white" /><span
                            class="mx-2 flex">GitHub</span
                        ></a
                    >
                </li>
            </ul>
        </nav>
    </header>
</Headroom>
{#if $achieve.isAchievementShown}
    <div class="fixed right-4 top-28 z-40" transition:fly={{ x: 300, duration: 2000 }}>
        <Achievement title="We Need To Go Deeper!" message="Activate Dark/Nether Theme!" />
    </div>
{/if}
<main
    class="flex flex-col flex-1 relative transition-colors duration-200 dark:bg-gray-900 dark:text-gray-50 md:pt-20 pt-16"
    role="main"
>
    <slot />
</main>
<footer
    class="bg-green-600 dark:bg-nether-netherrack-dark dark:netherrack text-white sm:flex sm:items-center justify-center transition-colors duration-500"
>
    <bold class="text-lg my-2 block sm:flex text-center">
        Made with <span class="dark:blue-heart sm:mx-1 text-red-600">&#10084;</span> by the Feather
        Association.
        <a href="credits" class="underline sm:mx-1">Credits</a>
    </bold>
    <DarkThemeSwitch class="mx-auto sm:mx-2 my-2 sm:my-0" />
</footer>

<style>
    /* li > a {
        @apply tracking-wide md:tracking-normal mx-auto hover:underline focus:text-blue-400 transition-colors;
    }
    li > a:not(.noaflex) {
        @apply flex;
    }
    li:not(.noaflex) {
        @apply flex;
    } */
</style>
