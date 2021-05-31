<script context="module">
    export async function preload({ query: { code } }) {
        return { code };
    }
</script>

<script lang="ts">
    import { API_BASE_URL } from "$lib/env"; 
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { token } from "$lib/stores/local";

    export let code: string;

    onMount(async () => {
        if ($token.secret) {
            await goto("/me");
        } else if (code) {
            const response = await fetch(
                `${API_BASE_URL}/me/authorization?code=${code}`
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
