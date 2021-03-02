<script context="module">
    import PluginVersion from "$components/PluginVersion.svelte"
    export async function preload({ params }) {
        const { name } = params;

		return { name };
	}
</script>

<script>
    export let name
</script>

<PluginVersion name={name} />