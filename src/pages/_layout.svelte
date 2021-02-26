<script>
    import { url } from '@roxi/routify'
    import Logo from "assets/chicken2.svg"
    import GithubMark from "assets/github-mark.svg"
</script>

<header class="flex bg-feather-dark text-white">
    <nav class="container mx-auto flex justify-between items-center h-16 sm:h-24 py-4 px-8">
        <a href="/" use:$url class="flex items-center"><Logo class="h-8 sm:h-12" /><span class="ml-3 text-4xl hidden sm:block">Feather</span></a>
        <ul class="flex items-center space-x-6 text-normal sm:text-xl font-bold">
            <li><a href="/plugins" use:$url>Plugins</a></li>
            <li><a href="/docs" use:$url>Docs</a></li>
            <li><a href="/faq" use:$url>FAQ</a></li>
            <li><a href="https://github.com/feather-rs/feather"><GithubMark class="h-8 sm:h-12 fill-current text-feather-light hover:text-white" /></a></li>
        </ul>
    </nav>
</header>
<main class="flex flex-col flex-1"><slot></slot></main>