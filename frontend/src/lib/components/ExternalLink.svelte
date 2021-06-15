<script lang="ts">
    export let href: string;
    let clazzName: string;
    export {clazzName as class};
</script>

<a href={href} class={clazzName} target="_blank" rel="noopener"><slot /></a>