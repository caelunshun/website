<script lang="ts">
    import { token } from "$stores/local";
	import Logo from "$assets/chicken2.svg";
	import GithubMark from "$assets/github-mark.svg"
    import { LockIcon, SettingsIcon, ChevronDownIcon } from "svelte-feather-icons"
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

<header class="flex bg-feather-dark text-white border-b border-gray-300">
    <nav class="md:flex flex-1 justify-between items-center md:h-24 py-4 px-12 relative {isDropdownShown ? "max-h-96" : "max-h-20 md:max-h-24"}" style="transition: max-height .5s;">
        <a href="/" class="flex items-center"><Logo class="h-8 sm:h-12" /><div class="ml-3 text-4xl block"><span>Feather</span></div></a>
        <ul class="md:flex items-center md:space-x-6 space-y-4 md:space-y-0 mt-4 md:mt-0 text-normal sm:text-xl font-bold {isDropdownShown ? "" : "hidden md:block"}">
            <li><a href="/association">Association</a></li>
            <li><a href="/plugins">Plugins</a></li>
            <li><a href="/docs">Docs</a></li>
            <li><a href="/faq">FAQ</a></li>
            <hr class="md:hidden">
            <!-- where should we put the link to the main git repo -->
            <!-- <li><a href="https://github.com/feather-rs/feather"><GithubMark class="h-8 sm:h-12 fill-current text-feather-light hover:text-white" /></a></li> -->
            <!-- Should be logged in users avatar -->
            {#if $token.secret}
            <li class="noaflex flex md:hidden"><a href="/me">
                <SettingsIcon class="h-6 w-6" />
                <span class="mx-2 my-auto">Account</span>
            </a></li>
            <li class="noaflex font-normal flex md:hidden"><a href="/me"><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Logout</span></a></li>
            <li bind:this={accountdropdownli} class="noaflex hidden md:flex cursor-pointer hover:underline relative" on:click={() => isAccountDropdownShown = !isAccountDropdownShown}>
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
                        <a href="/me" class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-t-lg">Profile</a>
                        <hr>
                        <a href="/me/logout" class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-b-lg">Logout</a>

                    </div>
                {/if}
            </li>
            {:else}
            <!-- How should the login button look? -->
            <li><a href="/me/login" class="flex items-center"><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Login (GitHub)</span></a></li>
            {/if}    
        </ul>
        <div class="absolute transition-transform right-6 top-9 md:hidden z-50 text-white transform -translate-x-1/2 -translate-y-1/2 {isDropdownShown ? "" : "rotate-90"}" on:click={() => {isDropdownShown = !isDropdownShown;}}>
            <ChevronDownIcon class="text-white h-10 w-10" />
        </div>
    </nav>
</header>
<main class="flex flex-col flex-1"><slot></slot></main>
<footer class="flex bg-feather-dark border-t border-gray-300"><h4 class="text-lg mx-auto my-2 text-white">Made with <span class="text-red-600">&#10084;</span> by the Feather Association</h4></footer>

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