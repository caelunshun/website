<script context="module">
    export async function preload({ params: { slug } }) {
        const response = await this.fetch('process.env.FEATHER_API/docs/summary')
        let markdown_html = await response.text();
        return { html: markdown_html };
    }
</script>

<script>
    export let html
</script>

<div class="flex w-full">
    <aside class="flex flex-shrink px-12 pb-8 bg-feather-dark text-white">
        <nav class="summary">
            {@html html}
        </nav>
    </aside>
    <article class="flex-grow flex-col py-24 px-8 text-lg">
        <slot></slot>
    </article>
</div>

<style>
</style>