<script>
    let clazz = undefined
    export { clazz as class }
</script>
<div class="p-4 shadow border-l-4 border-feather-light {clazz || ""}">
    <slot></slot>
</div>