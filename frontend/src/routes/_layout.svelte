<script lang="ts">
    import { token, preferences } from "$stores/local";
	import Logo from "$assets/chicken2.svg";
	import GithubMark from "$assets/github-mark.svg"
    import { LockIcon, SettingsIcon, MenuIcon, ChevronDownIcon } from "svelte-feather-icons"
    import { onMount } from "svelte";
    import {scale} from 'svelte/transition';

    let isDropdownShown: boolean = false;
    let isAccountDropdownShown: boolean = false;

    let accountdropdownli: HTMLLIElement = null;
    
    onMount(() => {
        const clicklistener = (event: Event) => {
            if(isAccountDropdownShown && !accountdropdownli.contains(event.target as HTMLElement)) {
                isAccountDropdownShown = false;
            }
        };
        const anchorclicklistener = () =>  {
            if(isDropdownShown) {
                isDropdownShown = false;
            }
        };
        document.addEventListener('click', clicklistener, false);
        for(let anchorelem of document.querySelectorAll('header a[href^="/"]')) {
            (anchorelem as HTMLAnchorElement).addEventListener('click', anchorclicklistener, false);
        }
        
    });

</script>

<header class="flex {$preferences.dark ? "bg-green-700" : "bg-green-600"} text-white sticky top-0 z-50 transition-colors duration-500">
    <nav class="md:flex flex-1 justify-between items-center md:h-24 py-4 px-4 container mx-auto relative {isDropdownShown ? "max-h-96" : "max-h-20 md:max-h-24"}" style="transition: max-height .5s;">
        <a href="/" class="flex items-center" tabindex={1}><Logo class="h-8 sm:h-12" /><div class="ml-3 text-4xl block"><span>Feather</span></div></a>
        <ul class="md:flex items-center md:space-x-6 space-y-4 md:space-y-0 mt-4 md:mt-0 text-normal sm:text-xl font-bold {isDropdownShown ? "" : "hidden md:block"}">
            <li><a href="/association" tabindex={1}>Association</a></li>
            <li><a href="/plugins" tabindex={1}>Plugins</a></li>
            <li><a href="/docs" tabindex={1}>Docs</a></li>
            <li><a href="/faq" tabindex={1}>FAQ</a></li>
            <hr class="md:hidden">
            <!-- where should we put the link to the main git repo -->
            <!-- <li><a href="https://github.com/feather-rs/feather"><GithubMark class="h-8 sm:h-12 fill-current text-feather-light hover:text-white" /></a></li> -->
            <!-- Should be logged in users avatar -->
            {#if $token.secret}
            <li class="noaflex flex md:hidden"><a href="/me" tabindex={1}>
                <SettingsIcon class="h-6 w-6" />
                <span class="mx-2 my-auto">Account</span>
            </a></li>
            <li class="noaflex font-normal flex md:hidden"><a href="/me/logout" tabindex={1}><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Logout</span></a></li>
            <li bind:this={accountdropdownli} class="noaflex hidden md:flex cursor-pointer hover:underline relative" tabindex={1} on:click={() => isAccountDropdownShown = !isAccountDropdownShown}>
                <span class="mx-2 my-auto">Account</span>
                <div class="my-auto">
                    <ChevronDownIcon class="text-white h-8 w-8" />
                </div>
                {#if isAccountDropdownShown}
                    <div
                        in:scale={{ duration: 100, start: 0.95}}
                        out:scale={{ duration: 75, start: 0.95}}
                        class="origin-top-right absolute right-0 w-48 mt-10 bg-gray-800 rounded-lg"
                        >
                        <a href="/me" class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-t-lg" tabindex={1}>Profile</a>
                        <hr>
                        <a href="/me/logout" class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-b-lg" tabindex={1}>Logout</a>

                    </div>
                {/if}
            </li>
            {:else}
            <!-- How should the login button look? -->
            <li><a href="/me/login" class="flex items-center" tabindex={1}><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Login (GitHub)</span></a></li>
            {/if}    
        </ul>
        <div class="absolute transition-transform right-6 top-9 md:hidden z-50 text-white transform -translate-x-1/2 -translate-y-1/2" tabindex={1} on:click={() => {isDropdownShown = !isDropdownShown;}}>
            <MenuIcon class="text-white h-10 w-10" />
        </div>
    </nav>
</header>
<main class="flex flex-col flex-1 transition-colors duration-200 {$preferences.dark ? "bg-gray-900 text-gray-50 dark" : ""}"><slot></slot></main>
<footer class="{$preferences.dark ? "bg-green-700" : "bg-green-600"} text-white sm:flex sm:items-center justify-center transition-colors duration-500">
    <bold class="text-lg my-2 block sm:flex text-center">
        Made with <span class="text-red-600 sm:mx-1">&#10084;</span> by the Feather Association. 
        <a href="credits" class="underline sm:mx-1" tabindex={100}>Credits</a>
        <!--span class="switch" checked={$preferences.dark} on:click={() => $preferences.dark = !$preferences.dark}>
            <span></span>
        </span-->
    </bold>
    <div class="w-16 h-8 flex items-center mx-auto sm:mx-2 my-2 sm:my-0 {$preferences.dark ? "bg-gray-900" : "bg-gray-300"} rounded-full cursor-pointer" tabindex={100} on:click={() => $preferences.dark = !$preferences.dark}>
        <div class="w-7 h-7 rounded-full shadow-md transform bg-white my-auto mx-1 {$preferences.dark ? "translate-x-7" : ""} duration-300 ease-in-out flex items-center justify-center">
            {#if $preferences.dark}
            🌛
            {:else}
            🌞
            {/if}
        </div>
    </div>
</footer>

<style>
    li > a {
        @apply tracking-wide md:tracking-normal mx-auto hover:underline focus:text-blue-400 transition-colors;
    }
    li > a:not(.noaflex) {
        @apply flex;
    }
    li:not(.noaflex) {
        @apply flex;
    }
</style>