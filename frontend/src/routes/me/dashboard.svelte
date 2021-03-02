<script>
    import Card from "$components/Card.svelte"
    import PluginCard from "$components/PluginCard.svelte"

    let plugins = [
        {
            id: "world-edit",
            name: "WorldEdit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            downloads: 420000,
            downloads_recent: 690,
            updated: 20,
            homepage: "https://google.com",
            repository: "https://github.com",
            versions: ["1.0.4", "1.13"]
        },
        {
            id: "world-edit",
            name: "WorldEdit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            downloads: 420000,
            downloads_recent: 690,
            updated: 20,
            versions: ["1.16"]
        },
        {
            id: "world-edit",
            name: "WorldEdit",
            description: "WorldEdit is an in-game map editor for both creative and survival",
            downloads: 420000,
            downloads_recent: 690,
            updated: 20,
            versions: ["1.16"]
        },
    ]
    let following = []
</script>

<div class="container mx-auto flex flex-col py-8 px-4">
    <h1 class="text-4xl font-bold">My Dashboard</h1>
    <div class="flex">
        <div class="mt-8">
            <h2 class="text-xl font-bold">My Plugins</h2>
            <ul>
                {#each plugins as plugin}
                    <li class="mt-4"><PluginCard {...plugin} /></li>
                {/each}
            </ul>
        </div>
    </div>
</div>