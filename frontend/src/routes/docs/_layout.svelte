<script context="module">
    import * as docscache from '$stores/docscache';

    export async function preload() {
        if(docscache.has("$summary")) {
            return { html: docscache.get("$summary") }
        } else {
            const response = await this.fetch('process.env.FEATHER_API/docs/summary')
            let markdown_html = await response.text();
            markdown_html = markdown_html.replaceAll("http://localhost:3000", "");
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

    const docanchorclicklistener = () => {
        if(isAsideShown) isAsideShown = false;
    }

    onMount(() => {
        for(let elem of document.querySelectorAll('a[href^="/docs/"]')) {
            (elem as HTMLAnchorElement).addEventListener('click', docanchorclicklistener, false);
        }

        return () => {
            for(let elem of document.querySelectorAll('a[href^="/docs/"]')) {
                (elem as HTMLAnchorElement).removeEventListener('click', docanchorclicklistener, false);
            }   
        }

    });

</script>


<div class="flex flex-1">
    <aside class="sp-flex-grow z-40 flex transition-transform transform md:translate-x-0 md:flex-shrink {isAsideShown ? "translate-x-0" : "-translate-x-full"} px-12 pb-8 bg-green-600 dark:bg-green-700 text-white md:static absolute">
        <nav class="summary">
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
        <div class="prose-sm md:prose xl:prose-xl dark:prose-dark markdown mx-auto md:my-24 my-6">
            <slot></slot>
        </div>
    </article>
</div>

<style>
    @media (max-width: 768px) {
        .sp-flex-grow {
            flex-grow: 1;
        }
    }
</style>