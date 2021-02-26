<script>
    import { params } from "@roxi/routify"
    import PluginVersion from "components/PluginVersion.svelte"
</script>

<PluginVersion id={$params.id} version={"1.0"} />