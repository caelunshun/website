<script>
    import { url, params, goto } from "@roxi/routify"
    import { SearchIcon } from "svelte-feather-icons"

    let search = ""
    let selected_categories = []
    $: $goto("/plugins", { search, categories: selected_categories })

    let plugins = [
        {
            id: "world-edit",
            name: "WorldEdit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            all_time: 420000,
            recent: 690,
            updated: 20,
            homepage: "https://google.com",
            repository: "https://github.com",
            versions: ["1.0.4", "1.13"]
        },
        {
            id: "world-edit",
            name: "WorldEdit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            all_time: 420000,
            recent: 690,
            updated: 20,
            versions: ["1.16"]
        },
        {
            id: "world-edit",
            name: "WorldEdit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            all_time: 420000,
            recent: 690,
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

<div class="container mx-auto flex flex-1 p-4 flex-col-reverse lg:flex-row">
    <div class="flex flex-1 flex-col my-4">
        <form class="flex justify-between items-center border border-feather-light w-full rounded text-xl px-4 py-2">
            <input class="flex-1" type="text" placeholder="Search..." bind:value={search} />
            <SearchIcon class="ml-4 h-6 w-6 text-feather-dark" />
        </form>
        <ul class="flex flex-col flex-1 space-y-4 mt-8">
            {#each plugins as { id, name, description, all_time, recent, updated, homepage, repository, versions }}
                <li class="flex flex-wrap justify-between p-4 shadow border-l-4 border-feather-light">
                    <div class="flex flex-col justify-between">
                        <div>
                        <div class="flex items-end">
                            <a href="/plugins/:id" 
                                use:$url={{ id }}
                                class="text-4xl font-bold hover:text-gray-600">
                                {name}
                            </a>
                            <ol class="flex">
                            {#each versions as version}
                                <li class="ml-2">{version}</li>
                            {/each}
                            </ol>
                        </div>
                        <p>{description}</p>
                        </div>
                        <ul class="flex space-x-2 mt-2">
                        {#if homepage}
                            <li><a href={homepage} class="font-bold border-b-2 border-feather-accent hover:text-gray-600">Homepage</a></li>
                        {/if}
                        {#if repository}
                            <li><a href={homepage} class="font-bold border-b-2 border-feather-accent hover:text-gray-600">Repository</a></li>
                        {/if}
                        </ul>
                    </div>
                    <div class="flex flex-col justify-center text-lg my-2">
                        <div>
                        <span><abbr title="Total numbers of downloads">All-Time:</abbr> {all_time.toLocaleString()}</span>
                        </div>
                        <div>
                        <span><abbr title="Downloads in the last 90 days">Recent:</abbr> {recent.toLocaleString()}</span>
                        </div>
                        <div>
                        <span><abbr title="The last time plugin was updated">Updated:</abbr> {updated.toLocaleString()} days ago</span>
                        </div>
                    </div>
                </li>
            {/each}
        </ul>
    </div>
    <aside class="border-0 lg:border-l-2 m-0 lg:mx-8 lg:pl-4 my-4">
        <h2 class="text-2xl font-bold">Categories</h2>
        <button on:click={() => { show_categories = !show_categories }} class="text-sm font-light lg:hidden hover:text-feather-dark">
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
                        <input type="checkbox" bind:group={selected_categories} value={category} />
                        <span class="ml-2 font-light text-sm">{category}</span>
                    </label>
                </li>
            {/each}
        </ul>
    </aside>
</div>