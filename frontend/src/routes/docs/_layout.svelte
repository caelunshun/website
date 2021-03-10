<div class="flex flex-1">
    <aside class="h-full px-8 bg-feather-dark text-white">
        <nav>
            <ol>
                <li>Introduction</li>
                <ol>
                    <ul>foo</ul>
                </ol>
            </ol>
        </nav>
    </aside>
    <article class="flex flex-1 flex-col">
        <slot></slot>
    </article>
</div>

<style>
ol {
    @apply text-2xl font-bold;
}
ol ol {
    @apply text-lg font-light pl-4 list-decimal list-inside;
}    
</style>