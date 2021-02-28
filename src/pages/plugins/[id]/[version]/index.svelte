<script>
    import { params } from "@roxi/routify"
    import PluginVersion from "components/plugin_version.svelte"
</script>

<PluginVersion id={$params.id} version={$params.version} />