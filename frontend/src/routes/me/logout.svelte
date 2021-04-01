<script lang="ts">
    import { onMount } from "svelte";
    import { token } from "$stores/local";

    onMount(async () => {
        $token = {};
        await goto("/")
    })
</script>