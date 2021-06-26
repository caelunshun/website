<script context="module">
	import PluginVersion from '$lib/components/PluginVersion.svelte';
	export async function load({ params }) {
		const { name, version } = params;

		return { props: { name, version } };
	}
</script>

<script>
	export let name;
	export let version;
</script>

<PluginVersion {name} {version} />
