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

    function toggleDarkMode() {
        $preferences.dark = !$preferences.dark;
    }

    function darkModeSwitchKeyPressHandler(event: KeyboardEvent) {
        if(event.code === "Space") toggleDarkMode();
    }

    let isEventChannelBlocked = false;

    function toggleAccountDropDown() {
        if(!isEventChannelBlocked) isAccountDropdownShown = !isAccountDropdownShown;
    }

    //Apparently Click is fired 50-100ms after focus
    function blockedEventChannel(e: FocusEvent | MouseEvent) {
        if(e instanceof FocusEvent) {
            isAccountDropdownShown = true;
            isEventChannelBlocked = true;
            setTimeout(() => isEventChannelBlocked = false, 100);
        } else {
            toggleAccountDropDown();
        }
    }

    let scrollY: number;
</script>

<svelte:window bind:scrollY={scrollY} />

<header 
    class="flex {$preferences.dark ? "bg-green-700" : "bg-green-600"} text-white sticky top-0 z-50 transition duration-500 {scrollY > 0 ? "shadow-xl" : ""}"
    role="heading"
    >
    <nav 
        class="md:flex flex-1 justify-between items-center md:h-24 py-4 px-4 container mx-auto relative {isDropdownShown ? "max-h-96" : "max-h-20 md:max-h-24"}" 
        style="transition: max-height .5s;"
        role="navigation"
    >
        <a href="/" class="flex items-center" ><Logo class="h-8 sm:h-12" /><div class="ml-3 text-4xl block"><span>Feather</span></div></a>
        <ul class="md:flex items-center md:space-x-6 space-y-4 md:space-y-0 mt-4 md:mt-0 text-normal sm:text-xl font-bold {isDropdownShown ? "" : "hidden md:block"}">
            <li><a href="/association" >Association</a></li>
            <li><a href="/plugins" >Plugins</a></li>
            <li><a href="/docs" >Docs</a></li>
            <li><a href="/faq" >FAQ</a></li>
            <hr class="md:hidden">
            <!-- where should we put the link to the main git repo -->
            <!-- <li><a href="https://github.com/feather-rs/feather"><GithubMark class="h-8 sm:h-12 fill-current text-feather-light hover:text-white" /></a></li> -->
            <!-- Should be logged in users avatar -->
            {#if $token.secret}
            <li class="noaflex flex md:hidden"><a href="/me">
                <SettingsIcon class="h-6 w-6" />
                <span class="mx-2 my-auto">Account</span>
            </a></li>
            <li class="noaflex font-normal flex md:hidden"><a href="/me/logout" ><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Logout</span></a></li>
            <li 
                bind:this={accountdropdownli} 
                class="noaflex hidden md:flex cursor-pointer hover:underline relative" 
                tabindex={0}
                on:keyup={(e) => {if(e.code === "Space") toggleAccountDropDown()}}
                on:focus={blockedEventChannel}
                on:click={blockedEventChannel}
            >
                <span class="mx-2 my-auto">Account</span>
                <div class="my-auto">
                    <ChevronDownIcon class="text-white h-8 w-8" />
                </div>
                {#if isAccountDropdownShown}
                    <div
                        in:scale={{ duration: 100, start: 0.95}}
                        out:scale={{ duration: 75, start: 0.95}}
                        class="origin-top-right absolute right-0 w-48 mt-10 bg-gray-800 rounded-lg border border-white"
                        >
                        <a href="/me" class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-t-lg">Profile</a>
                        <hr>
                        <a href="/me/logout" class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-b-lg" on:keydown={(e) => {if(e.code === "Tab") toggleAccountDropDown()}}>Logout</a>

                    </div>
                {/if}
            </li>
            {:else}
            <!-- How should the login button look? -->
            <li><a href="/me/login" class="flex items-center" ><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Login (GitHub)</span></a></li>
            {/if}    
        </ul>
        <div class="absolute transition-transform right-6 top-9 md:hidden z-50 text-white transform -translate-x-1/2 -translate-y-1/2"  on:click={() => {isDropdownShown = !isDropdownShown;}}>
            <MenuIcon class="text-white h-10 w-10" />
        </div>
    </nav>
</header>
<main 
    class="flex flex-col flex-1 transition-colors duration-200 {$preferences.dark ? "bg-gray-900 text-gray-50 dark" : ""}" 
    role="main"
>
    <slot></slot>
</main>
<footer class="{$preferences.dark ? "bg-green-700" : "bg-green-600"} text-white sm:flex sm:items-center justify-center transition-colors duration-500">
    <bold class="text-lg my-2 block sm:flex text-center">
        Made with <span class="text-red-600 sm:mx-1">&#10084;</span> by the Feather Association. 
        <a href="credits" class="underline sm:mx-1" >Credits</a>
        <!--span class="switch" checked={$preferences.dark} on:click={() => $preferences.dark = !$preferences.dark}>
            <span></span>
        </span-->
    </bold>
    <div 
        class="w-16 h-8 flex items-center mx-auto sm:mx-2 my-2 sm:my-0 {$preferences.dark ? "bg-gray-900" : "bg-gray-300"} rounded-full cursor-pointer" 
         
        role="switch"
        aria-checked={$preferences.dark}
        aria-label="Toggle Dark Mode"
        on:keyup={darkModeSwitchKeyPressHandler}
        on:click={toggleDarkMode}
    >
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