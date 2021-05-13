<script lang="ts">
import FeatherStartTerminalWrapper from "$components/FeatherStartTerminalWrapper.svelte";
import GithubMark from "$assets/github-mark.svg";
import {DownloadIcon} from "svelte-feather-icons";

import {preferences} from "$stores/local";
import {goto} from "@sapper/app";
</script>


<section class="w-full mx-auto px-4 py-4 md:py-32 flex flex-wrap sm:flex-nowrap items-center justify-center {$preferences.dark ? "bg-gray-800" : "bg-blue-50"}">
    <div class="flex flex-col m-8 sm:m-8">
        <h1 class="text-8xl font-bold font-display text-feather-accent">Feather</h1>
        <p class="text-2xl typography">A highly concurrent, performant, and extensible Minecraft server, powered by 
            <a href="https://www.rust-lang.org/">Rust</a>.</p>
    </div>
    <button 
        class="bg-green-600 dark:bg-green-700 text-white font-bold px-8 py-6 text-2xl flex rounded m-4 sm:m-8" 
        on:click={async () => await goto("download")}
    >
        Download <DownloadIcon class="w-8 h-8 ml-4 my-auto" />
    </button>
    
</section>
<!--section class="container mx-auto items-center justify-center py-4 text-white">
    <FeatherStartTerminalWrapper />
</section-->
<section class="typography">
    <div class="container mx-auto flex flex-col py-16">
        <h1 class="text-6xl m-4 sm:m-8">Why Feather?</h1>
        <div class="flex flex-wrap lg:flex-nowrap">
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">It's wicked fast</h2>
                <p class="mt-4 text-lg">
                    Not "stupid fast." Wicked fast. Feather's architecture is built from the ground up for performance
                    and elegance.
                    Written in Rust &mdash; a native language as fast as C &mdash; and based on the entity-component-system
                    architecture,
                    Feather can achieve performance no Vanilla fork could ever imagine.
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">Powerful plugins</h2>
                <p class="mt-4 text-lg">
                    Feather plugins can achieve incredible flexibility through Feather's data-oriented API.
                    Our plugin system has been written from the start to support any functionality ima-ginable,
                    to the extent that all gameplay features could be implemented as plugins if needed.
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">More flexability</h2>
                <p class="mt-4 text-lg">
                    Thanks to Feather's modular architec-ture, users can configure gameplay features to meet their needs.
                    Without writing code, you can configure Feather to the exact needs of your server, with-out
                    sacrificing performance.
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl flex">Open Source <GithubMark class="mx-2 {$preferences.dark ? "text-gray-50" : "text-black"} fill-current h-8" /></h2>
                <p class="mt-4 text-lg">
                    <!--Please add lots of irrelevant text here so that it looks good.-->
                    Feather is Open Source and the Source code can be browsed on 
                    <a href="https://github.com/feather-rs/feather">GitHub</a>. Feel free to contribute!
                    <!--span style="color: transparent;">Just a bunch of a's to test te alignment: aaaaa aaaaaaaaa aaaaaa aaaaaa 
                    aaaaaaa aaaaaa aaaaaa aaaaaa aaaaaaa aaaaaaaa aaaaaaaaaaaa aaaaaaaaa aaaaaaa aaaaaaa aaaa</span-->
                </p>
            </div>
        </div>
    </div>
</section>
<section class="typography {$preferences.dark ? "bg-gray-800" : "bg-blue-50"}">
    <div class="container mx-auto flex flex-col py-16">
        <h1 class="text-6xl m-4 sm:m-8">Why Rust?</h1>
        <div class="flex flex-wrap lg:flex-nowrap">
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">Performance</h2>
                <p class="mt-4 text-lg">
                    Rust is blazingly fast and memory-efficient: with no runtime or garbage collector,
                    it can power performance-critical services, run on embedded devices,
                    and easily integrate with other languages.
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">Reliability</h2>
                <p class="mt-4 text-lg">
                    Rust’s rich type system and ownership model guarantee memory-safety and thread-safety &mdash; preventing
                    many classes of bugs at compile-time.
                </p>
            </div>
        </div>
    </div>
</section>
<section class="typography">
    <div class="container mx-auto flex flex-col py-16">
        <h1 class="text-6xl m-4 sm:m-8">Get involved</h1>
        <div class="flex flex-wrap lg:flex-nowrap">
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">Issue tracker</h2>
                <p class="mt-4 text-lg">
                    Want to contribute? Check out our issue tracker to look for features which appeal to you.
                    Even if you're not a developer, feel free to share what you want to see in Feather's feature.It
                    means a lot to us!
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">Discord</h2>
                <p class="mt-4 text-lg">
                    Our Discord server is our central information hub. We'd love to hear your thoughts on Feather! Here you get information about new releases, 
                    can ask questions about develo-ping or find people that you can put in your server team! 
                    <a href="https://discordapp.com/invite/4eYmK69">Join!</a>
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">Association</h2>
                <p class="mt-4 text-lg">
                    Feather is run and funded by an open nonprofit association, where everyone is free to 
                    <a href="/association/join" >join at the cost of 10 EUR</a>.
                    All the members are invited once a year by the end of ___ to elect five to ten maintainers.
                    These maintainers are responsible for the daily mainte-nance of the association and the development
                    of feather.
                    All the money raised through the association goes towards paying for web and demo server; you can
                    read more about it in the <a href="/association/budget">budget</a>.
                </p>
            </div>
        </div>
    </div>
</section>

<svelte:head>
	<title>Feather | Wicked Fast Minecraft</title>
</svelte:head>

<style>
    /*section:nth-child(odd) {
        @apply bg-blue-50;
    }*/
    section > div > div > div > p {
        max-width: 50ch;
    }
</style>