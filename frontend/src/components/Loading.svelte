<script>
    import Logo from "$assets/chicken2.svg"
</script>

<div class="flex flex-1 justify-center items-center">
    <div class="heartbeat">
        <Logo class="h-20 w-20" />
    </div>
</div>

<style>
@keyframes heartbeat
{
    0%
    {
        transform: scale( .75 );
    }
    20%
    {
        transform: scale( 1 );
    }
    40%
    {
        transform: scale( .75 );
    }
    60%
    {
        transform: scale( 1 );
    }
    80%
    {
        transform: scale( .75 );
    }
    100%
    {
        transform: scale( .75 );
    }
}

.heartbeat {
    animation: heartbeat 1s infinite;
}
</style>