<script>
    import { Router } from "@roxi/routify"
    import { routes } from "../.routify/routes"
</script>

<style global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;

    html, body, #routify-app {
        @apply h-full flex flex-col;
    }

    .typography a {
        @apply border-b-2 border-feather-accent font-bold;
    }

    .typography.bg-feather-dark h1,
    .typography.bg-feather-dark h2,
    .typography.bg-feather-dark h3
    {
        @apply text-feather-light;
    }

    .typography h1,
    .typography h2,
    .typography h3,
    .typography h4
    {
        @apply font-bold font-display;
    }
</style>

<Router {routes} />
