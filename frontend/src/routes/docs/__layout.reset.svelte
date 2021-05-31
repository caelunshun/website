<script context="module">
    import { API_BASE_URL } from "$lib/env"; 
    import * as docscache from '$lib/stores/docscache';

    export async function load() {
        if(docscache.has("$summary")) {
            return { html: docscache.get("$summary") }
        } else {
            const response = await this.fetch(`${API_BASE_URL}/docs/summary`)
            let markdown_html = await response.text();
            markdown_html = markdown_html.replaceAll("https://raw.githubusercontent.com/Defman/feather/Docs/docs/src/", "/docs/");
            docscache.set("$summary", markdown_html);
            return { html: markdown_html };
        }
    }
</script>

<script lang="ts">
    import {MenuIcon, XIcon} from 'svelte-feather-icons';
    import {onMount} from 'svelte';
    export let html: string;
    let isAsideShown: boolean = false;

    let summaryElem: HTMLElement;
    let docElem: HTMLElement;

    let headings: {name: string, elem: HTMLElement}[] = [];

    const docanchorclicklistener = () => {
        if(isAsideShown) isAsideShown = false;
    }

    onMount(() => {
        for(let elem of summaryElem.querySelectorAll('a[href^="/docs/"]')) {
            (elem as HTMLAnchorElement).addEventListener('click', docanchorclicklistener, false);
        }

        for(let elem of docElem.querySelectorAll('*[id^="h-"]')) {
            headings = [...headings, { name: (elem as HTMLElement).innerText, elem: (elem as HTMLElement) }];
        }

        return () => {
            for(let elem of summaryElem.querySelectorAll('a[href^="/docs/"]')) {
                (elem as HTMLAnchorElement).removeEventListener('click', docanchorclicklistener, false);
            }   
        }

    });

</script>


<div class="flex flex-1">
    <aside class="summaryaside md:translate-x-0 {isAsideShown ? "translate-x-0" : "-translate-x-full"} bg-green-600 dark:bg-green-700 text-white">
        <nav class="summary" bind:this={summaryElem}>
            {@html html}
        </nav>
        <div class="md:hidden absolute top-8 right-8" on:click={() => isAsideShown = false}>
            <XIcon class="w-12 h-12 p-1 border border-gray-300 rounded-lg" />
        </div>
    </aside>
    <article class="flex flex-1 flex-col px-8 text-lg overflow-x-scroll md:overflow-x-visible">
        <div class="block md:hidden z-30 mt-4" on:click={() => isAsideShown = true}>
            <MenuIcon class="md:hidden text-white fill-current h-10 w-10 bg-green-600 dark:bg-green-800 rounded-lg p-2 border border-gray-300" />
        </div>
        <div class="prose-sm md:prose xl:prose-xl dark:prose-dark markdown mx-auto md:my-24 my-6" bind:this={docElem}>
            <slot></slot>
        </div>
    </article>
    <aside class="hidden md:block border-l border-black dark:border-white pl-4 py-4 relative overflow-y-hidden w-1/5">
        <h3 class="text-2xl fixed">Topics</h3>
        <ul class="block mt-8 fixed overflow-y-auto max-h-120 pr-14">
            {#each headings as heading, idx (idx)}
                <li class="block text-lg cursor-pointer px-2 my-1 border-l border-blue-500 transition-colors hover:bg-blue-500 duration-300 hover:text-black" on:click={() => {heading.elem.scrollIntoView({behavior: "smooth", block: "center"});}}>{heading.name}</li>
            {/each}
        </ul>
    </aside>
</div>

<style>
    .summaryaside {
        @apply flex-grow md:flex-grow-0 z-40 flex transition-transform transform md:flex-shrink px-12 pb-8 md:static absolute;
    }
</style>