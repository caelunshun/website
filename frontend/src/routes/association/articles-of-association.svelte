<script context="module">
    export async function preload({ params: { slug } }) {
        const response = await this.fetch('process.env.FEATHER_API/association/articles-of-association')
        let markdown_html = await response.text();
        return { html: markdown_html };
    }
</script>

<script>
    export let html
</script>

<div class="container mx-auto prose prose-sm sm:prose xl:prose-xl markdown py-32">
    {@html html}
</div>

