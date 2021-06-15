<script context="module" lang="ts">
    import { API_BASE_URL } from "$lib/env"; 
    import * as docscache from '$lib/stores/docscache';
    import type {DocResponse} from "$lib/types";
	
    /** @type {import('@sveltejs/kit').Load} */
    export async function load({fetch}) {
        if(docscache.has("$summary")) {
            return { props: {summaryHtml: docscache.get("$summary")} }
        } else {
            const response_fetch = await fetch(`${API_BASE_URL}/docs/summary`)
            let response_str = await response_fetch.text();
            let response: DocResponse = JSON.parse(response_str);
            let markdown_html = response.html;
            markdown_html = markdown_html.replace(/https\:\/\/raw\.githubusercontent\.com\/Defman\/feather\/Docs\/docs\/src\//g, "/docs/");
            docscache.set("$summary", markdown_html);
            return { props: {summaryHtml: markdown_html} };
        }
    }
</script>

<script lang="ts">
    import {MenuIcon, XIcon, ArrowLeftIcon} from 'svelte-feather-icons';
    import {docheadings} from "$lib/stores/local";
    import {onMount} from 'svelte';
    export let summaryHtml: string;
    let isAsideShown: boolean = false;

    let summaryElem: HTMLElement;
    let docElem: HTMLElement;

    let topics = [];

    const docanchorclicklistener = () => {
        if(isAsideShown) isAsideShown = false;
    }

    onMount(() => {
        summaryElem.querySelectorAll('a[href^="/docs/"]').forEach(elem =>
            (elem as HTMLAnchorElement).addEventListener('click', docanchorclicklistener, false)
        );

        return () => {
            summaryElem.querySelectorAll('a[href^="/docs/"]').forEach(elem =>
                (elem as HTMLAnchorElement).removeEventListener('click', docanchorclicklistener, false)
            )   
        }

    });

</script>


<div class="flex flex-1">
    <aside class="summaryaside transform md:translate-x-0 {isAsideShown ? "translate-x-0" : "-translate-x-full"} bg-green-600 dark:bg-green-700 text-white">
        <div class="flex justify-between items-center my-3">
            <a href="/" class="flex items-center">
                <ArrowLeftIcon class="mr-2 h-8 w-8" />
                <h4 class="text-xl">Go back</h4>
            </a>
            <button class="md:hidden" on:click={() => isAsideShown = false}>
                <XIcon class="w-10 h-10 p-1 border border-gray-300 rounded-lg" />
            </button>
        </div>
        <nav class="summary" bind:this={summaryElem}>
            {@html summaryHtml}
        </nav>
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
            {#each $docheadings as heading, idx (idx)}
                <li class="block text-lg cursor-pointer px-2 my-1 border-l border-blue-500 transition-colors hover:bg-blue-500 duration-300 hover:text-black"><a href={"#" + heading.hash}>{heading.name}</a></li>
            {/each}
        </ul>
    </aside>
</div>

<style>
    .summaryaside {
        @apply flex-grow flex-col md:flex-grow-0 z-40 flex transition-transform md:flex-shrink px-12 pb-8 md:static absolute w-screen md:w-auto;
    }
</style>