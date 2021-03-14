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

<div class="text-lg container mx-auto px-4 py-4 md:py-32 sm:flex-nowrap justify-center items-center markdown">
    {@html html}
</div>

