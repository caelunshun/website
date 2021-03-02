<script>
    import Logo from "$assets/chicken2.svg"

    let summary = fetch("SUMMARY.md")
</script>

<div class="flex flex-1">
    <aside class="flex flex-col bg-feather-dark w-64 shadow px-4 py-8 text-white">
        <a href="/" class="flex items-center"><Logo class="h-8 sm:h-12" /><span class="ml-3 text-4xl hidden sm:block">Feather</span></a>
        <ul class="mt-8">
            <li><a href="/">Back</a></li>
        </ul>
    </aside>
    <section class="flex flex-1 px-4 py-8">
        <slot></slot>
    </section>
</div>