<script lang="ts">
import Terminal from "../components/Terminal.svelte";
import type {Output} from '../types';
function rn() {
    let date = new Date();
    return `${date.getFullYear()}-${date.getMonth()+1}-${date.getDate()} ${date.getHours()}:${date.getMinutes()}:${date.getSeconds()},${date.getMilliseconds()} `
}
let outputs: Output[] = [
    {
        timeout: 1000,
        message: () => "feather-test$ ",
        newline: true,
        noslow: true
    },
    {
        timeout: 2000,
        message: () => "./feather-server",
        newline: false,
        typewriter: true,
        noslow: true
    },
    {
        timeout: 0,
        message: () => "Loading configuration",
        newline: true
    },
    {
        timeout: 0.38,
        message: () => (rn() + "INFO  [feather_server] Creating server"),
        newline: true
    },
    {
        timeout: 0.3,
        message: () => (rn() + "INFO  [feather_server] Server is listening on 0.0.0.0:25565"),
        newline: true
    },
    {
        timeout: 0.1,
        message: () => (rn() + "INFO  [feather_common::world_source::region] Chunk worker started"),
        newline: true
    },
    {
        timeout: 6700,
        message: () => "^C",
        newline: true,
        noslow: true
    },
    {
        timeout: 100,
        message: () => "feather-test$ ",
        newline: true,
        noslow: true
    },
    {
        timeout: 700,
        message: () => "clear",
        newline: false,
        typewriter: true,
        noslow: true
    }
];
</script>


<section class="container mx-auto px-4 py-4 md:py-32 flex flex-wrap sm:flex-nowrap items-center justify-center">
    <div class="flex flex-col m-8 sm:m-8">
        <h1 class="text-8xl font-bold font-display text-feather-accent">Feather</h1>
        <p class="text-2xl typography">A highly concurrent, performant, and extensible Minecraft server, powered by <a
                href="https://www.rust-lang.org/">Rust</a>.</p>
    </div>
    <button class="bg-feather-light text-feather-accent font-bold px-8 py-6 m-4 sm:m-8 text-2xl">Download</button>
    
</section>
<section class="container mx-auto items-center justify-center py-4">
    <Terminal outputs={outputs} />
</section>
<section class="bg-feather-dark text-white typography">
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
                    Thanks to Feather's modular architecture, users can configure gameplay features to meet their needs.
                    Without writing code, you can configure Feather to the exact needs of your server, with-out
                    sacrificing performance.
                </p>
            </div>
        </div>
    </div>
</section>
<section class="typography">
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
<section class="bg-feather-dark text-white typography">
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
                    can ask questions about develo-ping or find people that you can put in your server team! <a href="https://discordapp.com/invite/4eYmK69">Join!</a>
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h2 class="text-3xl">Association</h2>
                <p class="mt-4 text-lg">
                    Feather is run and funded by an open nonprofit association, where everyone is free to 
                    <a href="/association/join">join at the cost of 10 EUR</a>.
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