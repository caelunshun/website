<script lang="ts">
    import FlintAndSteel from "$lib/assets/minecraft/flint_and_steel.svg?component";
    export let title: string;
    export let message: string;
</script>

<div class="relative flex">
    <div class="corner absolute left-0 top-0 w-2 h-2" />
    <div class="corner absolute left-0 bottom-0 w-2 h-2 transform -rotate-90" />
    <div class="corner absolute right-0 top-0 w-2 h-2 transform rotate-90" />
    <div class="corner absolute right-0 bottom-0 w-2 h-2 transform rotate-180" />
    <div class="border-2 border-black">
        <div
            class="flex border-2 py-2 px-3 border-achievement-border items-center bg-achievement-background"
        >
            <div class="flintandsteel mr-3"><FlintAndSteel class="w-14 h-14" /></div>
            <div>
                <h1 class="pixelfont text-3xl text-yellow-300">{title}</h1>
                <h2 class="pixelfont text-2xl text-white">{message}</h2>
            </div>
        </div>
    </div>
</div>

<style lang="postcss">
    .corner {
        background-image: url(data:image/webp;base64,UklGRmAAAABXRUJQVlA4WAoAAAAQAAAAAwAAAwAAQUxQSBEAAAABDzD/ERFCIYAAKBJoIvof9ABWUDggKAAAAJABAJ0BKgQABAACADQlpAAC50WsAAD++gx+PY2orKBtnzCoXHaAAAA=);
        image-rendering: pixelated;
        background-size: cover;
    }
    @font-face {
        font-family: "Minecraft";
        font-style: normal;
        src: url("../assets/fonts/Minecraft.otf") format("opentype");
    }
    .pixelfont {
        font-family: "Minecraft";
    }
</style>
