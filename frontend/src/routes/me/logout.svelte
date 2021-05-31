<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { token } from "$lib/stores/local";

    onMount(async () => {
        $token = {};
        await goto("/")
    })
</script>