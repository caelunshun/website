<script context="module">
    export async function preload() {
        const response = await this.fetch('process.env.FEATHER_API/association/articles-of-association')
        let markdown_html = await response.text();
        return { html: markdown_html };
    }
</script>

<script>
    export let html
</script>

<div class="mx-4 md:w-10/12 md:mx-auto prose prose-sm sm:prose xl:prose-xl markdown my-10 md:my-32">
    {@html html}
</div>

