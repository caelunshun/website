<script>
    import { url } from "@roxi/routify"
    import Card from "components/card.svelte"

    export let id
    export let name
    export let description
    export let versions
    export let downloads
    export let downloads_recent
    export let updated
    export let homepage = undefined
    export let repository = undefined
</script>

<Card>
    <div class="flex flex-wrap justify-between">
        <div class="flex flex-col justify-between">
            <div class="mr-4">
                <div class="flex items-end">
                    <a href="/plugins/:id" 
                        use:$url={{ id }}
                        class="text-2xl font-bold hover:text-gray-600">
                        {name}
                    </a>
                    <ol class="flex">
                    {#each versions as version}
                        <li class="ml-1 text-sm">{version}</li>
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
        <div class="flex flex-col justify-center text-lg">
            <div>
            <span><abbr title="Total numbers of downloads">All-Time:</abbr> {downloads.toLocaleString()}</span>
            </div>
            <div>
            <span><abbr title="Downloads in the last 90 days">Recent:</abbr> {downloads_recent.toLocaleString()}</span>
            </div>
            <div>
            <span><abbr title="The last time plugin was updated">Updated:</abbr> {updated.toLocaleString()} days ago</span>
            </div>
        </div>
    </div>
</Card>