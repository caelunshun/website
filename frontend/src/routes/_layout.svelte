<script>
    import { token } from "$stores/local";
	import Logo from "$assets/chicken2.svg";
	import GithubMark from "$assets/github-mark.svg"
    import { LockIcon, SettingsIcon } from "svelte-feather-icons"
    import { onMount } from "svelte";

    let isDropdownShown = false;

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
            <li><a href="/me"><SettingsIcon class="h-6 w-6" /><span class="mx-2 my-auto">Account</span></a></li>
            <hr class="md:hidden w-4/5 mx-auto">
            <li noflex class="font-normal flex md:hidden"><a href="/me"><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Logout</span></a></li>
            {:else}
            <!-- How should the login button look? -->
            <li><a href="/me/login" class="flex items-center"><LockIcon class="h-6 w-6" /><span class="mx-2 my-auto">Login (GitHub)</span></a></li>
            {/if}    
        </ul>
        <div class="absolute right-6 top-9 md:hidden z-50 text-white transform -translate-x-1/2 -translate-y-1/2 {isDropdownShown ? "" : "rotate-90"}" on:click={() => {isDropdownShown = !isDropdownShown;}}>
            <svg height='25px' width='25px'  fill="#ffffff" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.0" x="0px" y="0px" viewBox="0 0 100 100" enable-background="new 0 0 100 100" xml:space="preserve"><g><path d="M47,73.7L6.5,33.2C4.8,31.5,4.5,28.8,6,27c1.7-2.1,4.8-2.2,6.7-0.3L47,61c1.6,1.6,4.3,1.6,5.9,0l34.3-34.3   c1.8-1.8,4.6-1.8,6.4,0c1.8,1.8,1.8,4.6,0,6.4L53,73.7C51.3,75.4,48.7,75.4,47,73.7z"></path></g></svg>
        </div>
    </nav>
</header>
<main class="flex flex-col flex-1"><slot></slot></main>

<style>
    li > a {
        @apply flex tracking-wide md:tracking-normal mx-auto;
    }
    li:not([noflex]) {
        @apply flex;
    }
</style>