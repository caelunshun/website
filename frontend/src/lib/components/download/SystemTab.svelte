<script lang="ts">
    import { ChevronLeftIcon } from "svelte-feather-icons";
    import { browser } from "$app/env";

    export let name: string;
    export let OSIcon: any;
    export let curSelIndex: number;
    export let desSelIndex: number;
    export let setSelIndex: (newi: number) => void;
    export let setSpacer: (newh: number) => void;
    export let fill: boolean = true;

    let stuff: HTMLDivElement;

    function tabClick(index: number) {
        setSelIndex(index);
        window.requestAnimationFrame(() => {
            window.requestAnimationFrame(() => setSpacer(stuff ? stuff.clientHeight : 0));
        });
    }

    function keyupEvent(e: KeyboardEvent) {
        if (e.code === "Space") {
            setSelIndex(desSelIndex);
        } else if (e.code === "ArrowRight") {
            if (curSelIndex !== 4) {
                setSelIndex(curSelIndex + 1);
            }
        } else if (e.code === "ArrowLeft") {
            if (curSelIndex !== 0) {
                setSelIndex(curSelIndex - 1);
            }
        }
    }
</script>

<div
    class="flex mx-auto px-4 cursor-pointer transition-colors duration-300 hover:bg-gray-500 hover:bg-opacity-10 {curSelIndex ===
    desSelIndex
        ? 'border-b border-blue-500 text-blue-500'
        : ''}"
    tabindex={0}
    role="tab"
    id="os-tab-{desSelIndex}"
    aria-selected={curSelIndex === desSelIndex}
    aira-controls="os-tabpanel-{desSelIndex}"
    on:keyup={keyupEvent}
    on:click={() => setSelIndex(desSelIndex)}
>
    <OSIcon class="w-8 h-8 my-2 mx-2 {fill ? 'fill-current' : ''}" />
    <label class="text-xl my-auto cursor-pointer" for="os-tab-{desSelIndex}">{name}</label>
    <div class="md:hidden my-auto ml-auto mr-2">
        <ChevronLeftIcon
            class="w-8 h-8 transition-transform transform {curSelIndex === desSelIndex
                ? '-rotate-90'
                : ''}"
        />
    </div>
</div>
{#if curSelIndex === desSelIndex}
    <div
        id="os-tabpanel-{desSelIndex}"
        class="flex justify-center dlinstr"
        bind:this={stuff}
        role="tabpanel"
        aria-label={name}
    >
        <div class="prose-auto max-w-full md:max-w-5xl">
            <slot />
        </div>
    </div>
{/if}
