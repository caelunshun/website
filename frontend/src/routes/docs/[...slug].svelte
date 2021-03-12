<script context="module">
    export async function preload({ params: { slug } }) {
        const response = await this.fetch(`https://raw.githubusercontent.com/feather-rs/feather/main/docs/${encodeURI(slug)}.md`)
        let markdown_raw = await response.text();
        return { html: markdown_raw };
    }
</script>

<script>
    export let html
</script>
{@html html}

<style>

</style>