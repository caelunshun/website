<script context="module">
    export async function preload({ params: { slug } }) {
        const response = await this.fetch(`process.env.FEATHER_API/docs/summary`)
        let markdown_html = await response.text();
        return { html: markdown_html };
    }
</script>

<script>
    export let html
</script>

<div class="flex flex-1">
    <aside class="flex px-8 bg-feather-dark text-white">
        <nav class="summary">
            {@html html}
        </nav>
    </aside>
    <article class="flex flex-1 flex-col py-4 px-8 prose">
        <slot></slot>
    </article>
</div>

<style>
:global(.summary > h1:first-child) {
    @apply hidden;
}

:global(.summary h1) {
    @apply text-xl mt-8;
}

:global(.summary ul) {
    counter-reset: item;
    list-style-type: none;
}
:global(.summary ul ul) {
    @apply pl-4;
}
:global(.summary ul > li) { 
    counter-increment: item;
    @apply mt-4;
}
:global(.summary ul > li::before) { 
    content: counters(item, ".") ". ";
    @apply font-bold;
}
:global(.summary hr) {
    margin-top: 5px;
}
:global(article code) {
    background-color: #ecf0f1;
    padding: 2px;
    border-radius: 5px;
}
:global(article code::before, article code::after) {
    content: "" !important;
}
</style>