<script lang="ts">
    import { SearchIcon } from "svelte-feather-icons"
    import PluginCard from "$components/PluginCard.svelte"

    let search = ""
    let selected_categories = []

    let plugins = [
        {
            name: "world-edit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            downloads: 420000,
            downloads_recent: 690,
            updated: 20,
            homepage: "https://google.com",
            repository: "https://github.com",
            versions: ["1.0.4", "1.13"]
        },
        {
            name: "world-edit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            downloads: 420000,
            downloads_recent: 690,
            updated: 20,
            versions: ["1.16"]
        },
        {
            name: "world-edit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            downloads: 420000,
            downloads_recent: 690,
            updated: 20,
            versions: ["1.16"]
        },
    ]

    let show_categories = false
    let categories = [
        "Admin Tool",
        "Anti-Griefing Tools",
        "Chat Related",
        "Developer Tools",
        "Economy",
        "Fixes",
        "Fun",
        "General",
        "Informational",
        "Mechanics",
        "Miscellaneous",
        "Role Playing",
        "Teleportation",
        "Twitch Integration",
        "Website Administation",
        "World Editing and Mangement",
        "World Generators",
    ]

</script>

<svelte:head>
    <title>Plugins | Feather</title>
</svelte:head>

<div class="container mx-auto flex flex-1 p-4 flex-col-reverse lg:flex-row">
    <div class="flex flex-1 flex-col my-4">
        <form class="flex justify-between items-center border border-feather-light w-full rounded text-xl px-4 py-2">
            <input class="flex-1 dark:bg-gray-900" type="text" placeholder="Search..." bind:value={search} tabindex={2} />
            <SearchIcon class="ml-4 h-6 w-6 text-feather-dark" />
        </form>
        <ul class="flex flex-col flex-1 space-y-4 mt-8">
            {#each plugins as plugin}
                <li>
                    <PluginCard {...plugin} />
                </li>
            {/each}
        </ul>
    </div>
    <aside class="border-0 lg:border-l-2 m-0 lg:mx-8 lg:pl-4 my-4">
        <h2 class="text-2xl font-bold">Categories</h2>
        <button on:click={() => { show_categories = !show_categories }} class="text-sm font-light lg:hidden hover:text-feather-dark" tabindex={2}>
        {#if show_categories}
            Hide categories
        {:else}
            Show categories
        {/if}
        </button>
        <ul class="flex flex-wrap lg:flex-nowrap lg:flex-col {!show_categories ? "hidden" : "block"} lg:block" >
            {#each categories as category}
                <li class="mr-4 my-2">
                    <label class="flex items-center">
                        <input type="checkbox" bind:group={selected_categories} value={category} tabindex={2} />
                        <span class="ml-2 font-light text-sm">{category}</span>
                    </label>
                </li>
            {/each}
        </ul>
    </aside>
</div>