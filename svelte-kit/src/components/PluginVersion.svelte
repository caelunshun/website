<script>
    import { DownloadCloudIcon, UploadCloudIcon, CopyIcon } from "svelte-feather-icons"

    export let id
    export let version

    const plugin = {
        id: "world-edit",
        name: "WorldEdit",
        description: "WorldEdit is an in-game map editor for both creative and survival",
        readme: "",
        all_time: 420000,
        recent: 690,
        updated: 20,
        homepage: "https://google.com",
        repository: "https://github.com",
        dependencies: [],
        categories: [],
        keywords: [],
        license: "MIT",
        authors: ["sk89q"],
        owners: ["sk89q"],
        versions: ["1.0.4", "1.0.0", "0.5.0", "0.1.0"],
        downloads: ["wasm32", "windows-x86", "osx-x86", "osx-m1"],
        keywords: [],
        categories: ["Admin Tools", "World Mangement"],
    }
</script>

<div class="container mx-auto">
    <div class="flex flex-1 flex-col lg:flex-row justify-between px-4 py-8">
        <section class="flex flex-1 typography pb-8 lg:py-0">
        {#if plugin.readme}
            {@html plugin.readme}
        {:else}
            <div class="flex flex-1 justify-center items-center">
                <span class="border-b-2 border-feather-accent px-1 text-xl font-thin">
                    Readme is empty!
                </span>
            </div>
        {/if}
        </section>
        <section class="flex flex-col lg:w-80 border-feather-light border-l-0 border-t-2 lg:border-l-2 lg:border-t-0 lg:pl-4 py-4 lg:py-0">
            <h1 class="text-3xl font-bold font-display">{id}<span class="text-sm font-light">{version}</span></h1>
            <p class="font-thin">{plugin.description}</p>
            <div class="flex flex-col mt-4">
                <span class="text-sm font-light">Last updated</span>
                <span class="font-bold">{plugin.updated} days ago</span>
            </div>
            <div class="flex flex-col mt-4">
                <span class="text-sm font-light">Plugin size</span>
                <span class="font-bold">{plugin.updated} KB</span>
            </div>
            <div class="flex flex-col mt-8">
                <span class="text-lg font-bold">Install</span>
                <div
                    class="flex justify-betwteen border-2 border-feather-light px-2 py-1 rounded font-mono bg-white hover:bg-feather-light items-center">
                    <code class="flex flex-1 text-sm">quill install {id}</code>
                    <CopyIcon class="h-4 w-4" />
                </div>
            </div>
            <div class="flex flex-col mt-4">
                <span class="text-lg font-bold">Download</span>
                <ul class="flex flex-wrap">
                    {#each plugin.downloads as arch}
                        <li>
                            <a href="/plugins/:slug/:version/:arch" 
                                class="mr-2 border-b-2 border-feather-accent">{arch}</a>
                        </li>
                    {/each}
                </ul>
            </div>
            <div class="flex flex-col mt-4">
                <span class="text-lg font-bold">Owners</span>
                <ul class="flex flex-wrap">
                    {#each plugin.owners as owner}
                        <li>
                            <a href="/users/:id"
                                class="mr-2 border-b-2 border-feather-accent">{owner}</a>
                        </li>
                    {/each}
                </ul>
            </div>
            <div class="flex flex-col mt-4">
                <span class="text-lg font-bold">Authors</span>
                <ul class="flex flex-wrap">
                    {#each plugin.authors as author}
                        <li>{author}</li>
                    {/each}
                </ul>
            </div>
            <div class="flex flex-col mt-4">
                <span class="text-lg font-bold">Keywords</span>
                <ul class="flex flex-wrap">
                    {#each plugin.keywords as keyword}
                        <li>
                            <a href="/plugins"
                                class="mr-2 border-b-2 border-feather-accent">{keyword}</a>
                        </li>
                    {/each}
                </ul>
            </div>
            <div class="flex flex-col mt-4">
                <span class="text-lg font-bold">Categories</span>
                <ul class="flex flex-wrap">
                    {#each plugin.categories as category}
                        <li>
                            <a href="/plugins"
                                class="mr-2 border-b-2 border-feather-accent">{category}</a>
                        </li>
                    {/each}
                </ul>
            </div>
            <div class="flex flex-col mt-4">
                <span class="text-lg font-bold">Versions</span>
                <ul class="flex flex-wrap">
                    {#each plugin.versions as version}
                        <li>
                            <a href="/plugins"
                                class="mr-2 border-b-2 border-feather-accent">{version}</a>
                        </li>
                    {/each}
                </ul>
            </div>
            <div class="flex mt-8">
                <div class="flex items-center">
                    <abbr title="Total downloads of this plugin"><DownloadCloudIcon class="h-8 w-8" /></abbr><span class="ml-2 text-xl">600</span>
                </div>
                <div class="flex items-center ml-8">
                    <abbr title="Total published versions of this plugin"><UploadCloudIcon class="h-8 w-8" /></abbr><span class="ml-2 text-xl">4</span>
                </div>
            </div>
        </section>
    </div>
</div>