<script>
    import { params } from "@roxi/routify"
    $: id = $params.id
    $: version = $params.version
</script>

<h1>Docs for {id} {version}</h1>