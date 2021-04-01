<script>
    import { token } from "$stores/local";
	import Logo from "$assets/chicken2.svg";
	import GithubMark from "$assets/github-mark.svg"
    import { LockIcon, SettingsIcon } from "svelte-feather-icons"
    import { onMount } from "svelte";
    import DropDownSvg from "../components/DropDownSVG.svelte";

    let isDropdownShown = false;
    let isAccountDropdownShown = false;

</script>

<header class="flex bg-feather-dark text-white">
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
            {#if !$token.secret}
            <li noflex class="flex md:hidden"><a href="/me">
                <SettingsIcon class="h-6 w-6" />
                <span class="mx-2 my-auto">Account</span>
            </a></li>
            <li noflex class="font-normal flex md:hidden"><a href="/me"><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Logout</span></a></li>
            <li noflex class="hidden md:flex cursor-pointer hover:underline relative" on:focus={() => isAccountDropdownShown = true} on:blur={() => isAccountDropdownShown = false}>
                <span class="mx-2 my-auto">Account</span>
                <div class="my-auto">
                    <DropDownSvg color="ffffff" width={20} height={20} />
                </div>
                {#if isAccountDropdownShown}
                    <div></div>
                {/if}
            </li>
            {:else}
            <!-- How should the login button look? -->
            <li><a href="/me/login" class="flex items-center"><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Login (GitHub)</span></a></li>
            {/if}    
        </ul>
        <div class="absolute right-6 top-9 md:hidden z-50 text-white transform -translate-x-1/2 -translate-y-1/2 {isDropdownShown ? "" : "rotate-90"}" on:click={() => {isDropdownShown = !isDropdownShown;}}>
            <DropDownSvg color="ffffff" width={25} height={25} />
        </div>
    </nav>
</header>
<main class="flex flex-col flex-1"><slot></slot></main>

<style>
    li > a {
        @apply tracking-wide md:tracking-normal mx-auto hover:underline;
    }
    li > a:not([noflex]) {
        @apply flex;
    }
    li:not([noflex]) {
        @apply flex;
    }
</style>