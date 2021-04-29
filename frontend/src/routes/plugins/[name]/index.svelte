<script context="module">
    import PluginVersion from "$components/PluginVersion.svelte"
    export async function preload({ params: { name } }) {
		return { name };
	}
</script>

<script>
    export let name
</script>

<PluginVersion name={name} />

<svelte:head>
    <title>{name} | Feather Plugins</title>
</svelte:head>