<script>
    import { token } from "$stores/local";
	import Logo from "$assets/chicken2.svg";
	import GithubMark from "$assets/github-mark.svg"
    import { LockIcon, SettingsIcon } from "svelte-feather-icons"
    import { onMount } from "svelte";

</script>

<header class="flex bg-feather-dark text-white">
    <nav class="flex flex-1 justify-between items-center h-16 sm:h-24 py-4 px-12">
        <a href="/" class="flex items-center"><Logo class="h-8 sm:h-12" /><span class="ml-3 text-4xl hidden sm:block">Feather</span></a>
        <ul class="flex items-center space-x-6 text-normal sm:text-xl font-bold">
            <li><a href="/association">Association</a></li>
            <li><a href="/plugins">Plugins</a></li>
            <li><a href="/docs">Docs</a></li>
            <li><a href="/faq">FAQ</a></li>
            <!-- where should we put the link to the main git repo -->
            <!-- <li><a href="https://github.com/feather-rs/feather"><GithubMark class="h-8 sm:h-12 fill-current text-feather-light hover:text-white" /></a></li> -->
            <!-- Should be logged in users avatar -->
            <li><a href="/me"><SettingsIcon class="h-6 w-6" /></a></li>
            <!-- How should the login button look? -->
            <li><a href="/me/login" class="flex items-center"><LockIcon class="h-6 w-6" /></a></li>
        </ul>
    </nav>
</header>
<main class="flex flex-col flex-1"><slot></slot></main>
