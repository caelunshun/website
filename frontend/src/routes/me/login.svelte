<script context="module" lang="ts">
    import type { Load } from "@sveltejs/kit";
    export const load: Load = async ({ page: { query } }) => {
        return { props: { code: query.get("code") } };
    };
</script>

<script lang="ts">
    import { API_BASE_URL, GITHUB_IDENTITY } from "$lib/env";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { token } from "$lib/stores/local";

    export let code: string;

    onMount(async () => {
        if ($token.secret) {
            await goto("/me");
        } else if (code) {
            const response = await fetch(`${API_BASE_URL}/me/authorization?code=${code}`);
            if (response.status === 200) {
                const { secret } = await response.json();
                $token = { secret };
            } else {
                await goto("/me/failedtologin");
            }
            await goto("/me");
        } else {
            window.location.href = GITHUB_IDENTITY as string;
        }
    });
</script>
