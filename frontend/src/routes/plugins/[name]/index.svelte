<script context="module">
    export async function load({
        page: {
            params: { name }
        }
    }) {
        return { props: { name: name } };
    }
</script>

<script>
    import PluginVersion from "$lib/components/PluginVersion.svelte";
    export let name;
</script>

<PluginVersion {name} />

<svelte:head>
    <title>{name} | Feather Plugins</title>
</svelte:head>
