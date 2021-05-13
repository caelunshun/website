<script context="module">
    export async function preload() {
        const response = await this.fetch('process.env.FEATHER_API/association/budget')
        let markdown_html = await response.text();
        return { html: markdown_html };
    }
</script>

<script>
    export let html
</script>

<div class="mx-4 md:w-10/12 md:mx-auto prose-sm sm:prose dark:prose-dark xl:prose-xl markdown my-10 md:my-24">
    {@html html}
</div>