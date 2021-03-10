<script context="module">
    export async function preload({ query: { code } }) {
        return { code };
    }
</script>

<script>
    import { goto } from "@sapper/app";
    import { onMount } from "svelte";
    import { token } from "$stores/local.js";

    export let code;

    onMount(async () => {
        if ($token.secret) {
            await goto("/me");
        } else if (code) {
            const response = await fetch(
                `process.env.FEATHER_API/me/authorization?code=${code}`
            );
            if (response.status === 200) {
                const { secret } = await response.json();
                $token = { secret };
            }
            await goto("/me");
        } else {
            await goto("process.env.GITHUB_IDENTITY");
        }
    });
</script>
