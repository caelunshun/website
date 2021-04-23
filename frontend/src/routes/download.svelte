<script lang="ts">
    import AppleIcon from '$assets/os/apple.svg';
    import WindowsIcon from '$assets/os/windows.svg';
    import LinuxIcon from '$assets/os/linux.svg';
    import DockerIcon from '$assets/os/docker.svg';
    import {PackageIcon, ChevronLeftIcon} from 'svelte-feather-icons';

    let isMobile = false;

    let selectedTabIndex = -1;
    if(process.browser) {
        if(navigator.userAgent.indexOf("Win") != -1) selectedTabIndex = 2;
        else if(navigator.userAgent.indexOf("Android") != -1) {selectedTabIndex = -1; isMobile = true}
        else if(navigator.userAgent.indexOf("like Mac") != -1) {selectedTabIndex = -1; isMobile = true}
        else if(navigator.userAgent.indexOf("Linux") != -1) selectedTabIndex = 0;
        else if(navigator.userAgent.indexOf("Mac") != -1) selectedTabIndex = 1;
        else selectedTabIndex = 3;
    }

    function setTabIndex(newindex: number) {
        if(selectedTabIndex === newindex && isMobile) selectedTabIndex = -1;
        else selectedTabIndex = newindex;
    }
</script>

<div class="container mx-auto">
    <h2 class="text-4xl font-bold my-4">Download</h2>
    <hr>
    <h3 class="text-3xl font-semibold my-4">Platforms</h3>
    <div class="mb-4 md:flex flex-grow md:border-b border-gray-500 text-gray-500">
        <div class="flex mx-auto cursor-pointer {selectedTabIndex === 0 ? "border-b border-blue-500 text-blue-500" : ""}" on:click={() => setTabIndex(0)}>
            <LinuxIcon class="w-8 h-8 my-2 mx-2 fill-current" />
            <span class="text-xl my-auto">Linux</span>
            <div class="sm:hidden my-auto ml-auto mr-2">
                <ChevronLeftIcon class="w-8 h-8 transition-transform transform {selectedTabIndex === 0 ? "-rotate-90" : ""}" />
            </div>
        </div>
        {#if selectedTabIndex === 0}
            <div class="sm:hidden instructions">
                <p>Download for Linux</p>
            </div>
        {/if}
        <div class="flex mx-auto cursor-pointer {selectedTabIndex === 1 ? "border-b border-blue-500 text-blue-500" : ""}" on:click={() => setTabIndex(1)}>
            <AppleIcon class="w-8 h-8 my-2 mx-2 fill-current" />
            <span class="text-xl my-auto">MacOS</span>
            <div class="sm:hidden my-auto ml-auto mr-2">
                <ChevronLeftIcon class="w-8 h-8 transition-transform transform {selectedTabIndex === 1 ? "-rotate-90" : ""}" />
            </div>
        </div>
        {#if selectedTabIndex === 1}
            <div class="sm:hidden instructions">
                <p>Download for MacOS</p>
            </div>
        {/if}
        <div class="flex mx-auto cursor-pointer {selectedTabIndex === 2 ? "border-b border-blue-500 text-blue-500" : ""}" on:click={() => setTabIndex(2)}>
            <WindowsIcon class="w-8 h-8 my-2 mx-2 fill-current" />
            <span class="text-xl my-auto">Windows</span>
            <div class="sm:hidden my-auto ml-auto mr-2">
                <ChevronLeftIcon class="w-8 h-8 transition-transform transform {selectedTabIndex === 2 ? "-rotate-90" : ""}" />
            </div>
        </div>
        {#if selectedTabIndex === 2}
            <div class="sm:hidden instructions">
                <p>Download for Windows</p>
            </div>
        {/if}
        <div class="flex mx-auto cursor-pointer {selectedTabIndex === 3 ? "border-b border-blue-500 text-blue-500" : ""}" on:click={() => setTabIndex(3)}>
            <PackageIcon class="w-8 h-8 my-2 mx-2" />
            <span class="text-xl my-auto">From source</span>
            <div class="sm:hidden my-auto ml-auto mr-2">
                <ChevronLeftIcon class="w-8 h-8 transition-transform transform {selectedTabIndex === 3 ? "-rotate-90" : ""}" />
            </div>
        </div>
        {#if selectedTabIndex === 3}
            <div class="sm:hidden instructions">
                <p>Download and install from source</p>
            </div>
        {/if}
        <div class="flex mx-auto cursor-pointer {selectedTabIndex === 4 ? "border-b border-blue-500 text-blue-500" : ""}" on:click={() => setTabIndex(4)}>
            <DockerIcon class="w-8 h-8 my-2 mx-2 fill-current" />
            <span class="text-xl my-auto">Docker</span>
            <div class="sm:hidden my-auto ml-auto mr-2">
                <ChevronLeftIcon class="w-8 h-8 transition-transform transform {selectedTabIndex === 4 ? "-rotate-90" : ""}" />
            </div>
        </div>
        {#if selectedTabIndex === 4}
            <div class="sm:hidden instructions">
                <p>Image for Docker</p>
            </div>
        {/if}
    </div>
    <div class="instructions hidden sm:block">
        {#if selectedTabIndex === 0}
        <p>Download for Linux</p>
        {:else if selectedTabIndex === 1}
        <p>Download for MacOS</p>
        {:else if selectedTabIndex === 2}
        <p>Download for Windows</p>
        {:else if selectedTabIndex === 3}
        <p>Download and install from source</p>
        {:else if selectedTabIndex === 4}
        <p>Image for Docker</p>
        {/if}
    </div>
</div>

<style>
    .instructions {
        @apply text-gray-500;
    }
    .instructions p {
        @apply text-lg;
    }
    .instructions * {
        @apply mx-2;
    }
</style>