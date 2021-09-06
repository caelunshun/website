<script>
    import Logo from "$lib/assets/chicken2.svg?component";
</script>

<div class="flex flex-1 justify-center items-center h-full w-full">
    <div class="heartbeat">
        <Logo class="h-20 w-20 md:h-40 md:w-40" />
    </div>
</div>

<style>
    @keyframes heartbeat {
        0% {
            transform: scale(0.75);
        }
        20% {
            transform: scale(1);
        }
        40% {
            transform: scale(0.75);
        }
        60% {
            transform: scale(1);
        }
        80% {
            transform: scale(0.75);
        }
        100% {
            transform: scale(0.75);
        }
    }

    .heartbeat {
        animation: heartbeat 1s infinite;
    }
</style>
