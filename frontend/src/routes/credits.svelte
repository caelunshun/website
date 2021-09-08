<script lang="ts">
    import Link from "$lib/components/ExternalLink.svelte";
</script>

<div class="container mx-auto p-4 md:p-8">
    <h2 class="text-2xl font-bold my-4">Tools used to create this website.</h2>
    <hr class="my-2" />
    <div class="prose-auto md:prose-lg">
        <h3>
            Frontend <Link
                class="extlink"
                href="https://github.com/feather-rs/website/blob/master/frontend/package.json"
                >Full dependency list</Link
            >
        </h3>
        <ul>
            <li>Framework: SvelteKit</li>
            <li>Style: PostCSS & TailwindCSS</li>
            <li>
                Icons: svelte-feather-icons, FontAwesome & vite-plugin-svelte-svg (to import local
                icons)
            </li>
            <li>Additional: TypeScript & TimeAgo</li>
        </ul>
    </div>
    <!-- <div class="prose-auto md:prose-lg">
        <h3>
            Backend <Link
                class="extlink"
                href="https://github.com/feather-rs/website/blob/master/api/Cargo.toml"
                >Full dependency list</Link
            >
        </h3>
        <ul>
            <li>API-Server: warp</li>
            <li>Database: Postgres & sqlx</li>
            <li>Fetching data: reqwest</li>
            <li>Docs parsing: pulldown-cmark & syntect</li>
        </ul>
    </div> -->
</div>

<svelte:head>
    <title>Credits | Feather</title>
</svelte:head>
