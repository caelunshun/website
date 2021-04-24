<script lang="ts">
    import {ChevronLeftIcon} from "svelte-feather-icons";

    export let name: string;
    export let OSIcon: any;
    export let curSelIndex: number;
    export let desSelIndex: number;
    export let setSelIndex: (newi:number) => void;
    export let setSpacer: (newh: number) => void;
    export let fill: boolean = true;

    let stuff: HTMLDivElement;

    if(process.browser) {
        setTimeout(() => setSpacer(stuff ? stuff.clientHeight : 0), 50);
        setInterval(() => setSpacer(stuff ? stuff.clientHeight : 0), 1000);
    }
</script>

<div class="flex mx-auto px-4 cursor-pointer transition-colors duration-300 hover:bg-gray-500 hover:bg-opacity-10 {curSelIndex === desSelIndex ? "border-b border-blue-500 text-blue-500" : ""}" on:click={() => setSelIndex(desSelIndex)}>
    <OSIcon class="w-8 h-8 my-2 mx-2 {fill ? "fill-current" : ""}" />
    <span class="text-xl my-auto">{name}</span>
    <div class="md:hidden my-auto ml-auto mr-2">
        <ChevronLeftIcon class="w-8 h-8 transition-transform transform {curSelIndex === desSelIndex ? "-rotate-90" : ""}" />
    </div>
</div>
{#if curSelIndex === desSelIndex}
    <div class="dlinstr prose markdown" bind:this={stuff}>
        <slot></slot>
    </div>
{/if}