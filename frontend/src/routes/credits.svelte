<script lang="ts">
    import Link from "$lib/components/ExternalLink.svelte";
</script>

<div class="container mx-auto">
    <h2 class="text-2xl font-bold my-4">Tools used to create this website.</h2>
    <hr class="my-2" />
    <div class="credcontainer">
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
    <div class="credcontainer">
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
    </div>
</div>

<svelte:head>
    <title>Credits | Feather</title>
</svelte:head>

<style>
    .credcontainer {
        @apply my-4;
    }
    .credcontainer > h3 {
        @apply font-bold text-xl;
    }
    :global(.extlink) {
        @apply font-normal text-blue-500 cursor-pointer text-base;
    }
    .credcontainer > ul {
        @apply ml-8 list-disc;
    }
    .credcontainer > ul > li {
        @apply my-2;
    }
</style>
