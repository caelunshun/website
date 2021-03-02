<script context="module">
    import PluginVersion from "$components/PluginVersion.svelte"
	export async function preload({ params }) {
		const { name, version } = params;

		return { name, version };
	}
</script>

<script>
    export let name
    export let version
</script>


<PluginVersion name={name} version={version} />