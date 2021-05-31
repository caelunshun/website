<script lang="ts">
    import AppleIcon from '$lib/assets/os/apple.svg?component';
    import WindowsIcon from '$lib/assets/os/windows.svg?component';
    import LinuxIcon from '$lib/assets/os/linux.svg?component';
    import DockerIcon from '$lib/assets/os/docker.svg?component';
    import {PackageIcon} from 'svelte-feather-icons';
    import {browser} from "$app/env";

    import OSTab from "$lib/components/download/SystemTab.svelte";

    let isMobile = false;

    let selectedTabIndex = -1;
    if(browser) {
        if(navigator.userAgent.indexOf("Win") != -1) selectedTabIndex = 2;
        else if(navigator.userAgent.indexOf("Android") != -1) selectedTabIndex = -1;
        else if(navigator.userAgent.indexOf("like Mac") != -1) selectedTabIndex = -1;
        else if(navigator.userAgent.indexOf("Linux") != -1) selectedTabIndex = 0;
        else if(navigator.userAgent.indexOf("Mac") != -1) selectedTabIndex = 1;
        else selectedTabIndex = 3;
        let mq = window.matchMedia("(max-width: 768px)");
        isMobile = mq.matches;
        if(!isMobile && selectedTabIndex === -1) selectedTabIndex = 3 
        mq.addEventListener("change", (e) => {
            isMobile = e.matches;
        });
    }
    
    let spacerHeight: number = 0;
    function setTabIndex(newindex: number) {
        if(selectedTabIndex === newindex && isMobile) selectedTabIndex = -1;
        else selectedTabIndex = newindex;
        spacerHeight = 0;
    }

    function setSpacerHeight(newhe: number) {
        if(spacerHeight < newhe)
            spacerHeight = newhe;
    }

    let nixcmd = '<pre style="background-color:#0d1117;">\n'+
        '<span style="color:#ffa657;">curl --proto </span><span style="color:#a5d6ff;">&#39;=https&#39;</span><span style="color:#ffa657;"> --tlsv1</span><span style="color:#c9d1d9;">.2</span><span style="color:#ffa657;"> -sSf</span><span style="color:#c9d1d9;"> https://sh.feathermc.org </span><span style="color:#ff7b72;">| </span><span style="color:#ffa657;">sh</span></pre>';
        
</script>

<div class="container mx-auto">
    <h2 class="text-4xl font-bold my-4 mx-4 md:mx-0">Download</h2>
    <hr class="w-10/12 mx-auto md:w-full">
    <h3 class="text-3xl font-semibold my-4 mx-4 md:mx-0">Platforms</h3>
    <div class="md:relative md:flex flex-grow md:border-b border-gray-500 text-gray-500 dark:text-gray-50 w-full">
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={0} name="Linux" setSelIndex={setTabIndex} OSIcon={LinuxIcon} setSpacer={setSpacerHeight}>
            <h4>Download</h4>
            <p>To get Feather running on a Linux system first either run the download script:</p>
            {@html nixcmd}
            <p>or download the latest <a href="https://github.com/feather-rs/feather/releases/latest" target="_blank" rel="noopener" >GitHub release</a> and unzip it.</p>
            <h5>Alternative</h5>
            <p>Alternatively you can also build Feather <span class="sanchor text-gray-900 dark:text-gray-200" tabindex={0} on:click={() => {setTabIndex(3)}}>from source</span>. 
                This will guarantee that you have the latest version of Feather.</p>
            <h4>Running</h4>
            <p>Second simply head into its directory and run <code>./feather-server</code></p>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={1} name="MacOS" setSelIndex={setTabIndex} OSIcon={AppleIcon} setSpacer={setSpacerHeight}>
            <h4>Download</h4>
            <p>To get Feather running on a MacOS system first either run the download script:</p>
            {@html nixcmd}
            <p>or download the latest <a href="https://github.com/feather-rs/feather/releases/latest" target="_blank" rel="noopener" >GitHub release</a> and unzip it.</p>
            <h5>Alternative</h5>
            <p>Alternatively you can also build Feather <span class="sanchor text-gray-900 dark:text-gray-200" tabindex={0} on:click={() => {setTabIndex(3)}}>from source</span>. 
                This will guarantee that you have the latest version of Feather.</p>
            <h4>Running</h4>
            <p>Second simply head into its directory and run <code>./feather-server</code></p>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={2} name="Windows" setSelIndex={setTabIndex} OSIcon={WindowsIcon} setSpacer={setSpacerHeight}>
            <h4>Native Windows</h4>
            <h5>Download</h5>
            <button class="bg-blue-500 ring-blue-500 text-white rounded-lg px-4 py-2 ring-offset-2 hover:bg-blue-700 focus:bg-blue-700 transition-colors duration-200 active:ring" 
                tabindex={0}
                style="border: none; outline: none;" 
                on:click={() => window.open("https://github.com/feather-rs/feather/releases/latest/download/feather-v0.6.0-windows.zip")}
                >
                Downlaod
            </button>
            <p>To run Feather on a native Windows system first download its binaries from the latest 
                <a href="https://github.com/feather-rs/feather/releases/latest" target="_blank" rel="noopener" >GitHub release</a>.</p>
            <p>Alternative: Clone and build <span class="sanchor text-gray-900 dark:text-gray-200"  on:click={() => {setTabIndex(3)}}>from source</span>. 
                This will guarantee that you have the latest version of Feather.</p>
            <h5>Running</h5>
            <p>Second execute the <code>feather-server.exe</code> executable via a Terminal.</p>
            <h4>Subsystem</h4>
            <p>If you want to run Feather from a 
                <a href="https://docs.microsoft.com/en-gb/windows/wsl/install-win10" target="_blank" rel="noopener" >Windows Subsystem for Linux</a> 
                look at the download instructions for 
                <span class="sanchor text-gray-900 dark:text-gray-200" tabindex={0} on:click={() => {setTabIndex(0)}}>Linux</span> 
                and execute them inside of your Subsystem.</p>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={3} name="From source" setSelIndex={setTabIndex} OSIcon={PackageIcon} fill={false} setSpacer={setSpacerHeight}>
            <h4>Prerequirements</h4>
            <ul>
                <li><a href="https://rustup.rs/" target="_blank" rel="noopener">Rust</a> (latest toolchain version)</li>
            </ul>
            <h4>Cloning and Installing</h4>
            <p>To clone and install Feather from source do the following:</p>
            <ol>
                <li>Clone Feathers source code from <a href="https://github.com/feather-rs/feather" target="_blank" rel="noopener">here</a></li>
                <li>Go into the project directory and run <code>cargo build --release</code></li>
                <li>Wait for the build to finish. The finished version should be in <code>target/release/</code></li>
                <li>To start the server run <code>feather-server</code> (in a Terminal)</li>
            </ol>
            <h5>Script in one: </h5>
            <!--Idk why but it works this way but not if it is added in manually (maybe because svelte compiles it to rough)-->
            {@html '<pre style="background-color:#0d1117;">\n'+
            '<span style="color:#ffa657;">git</span><span style="color:#c9d1d9;"> clone https://github.com/feather-rs/feather\n'+
            '</span><span style="color:#d2a8ff;">cd</span><span style="color:#c9d1d9;"> feather\n'+
            '</span><span style="color:#ffa657;">cargo</span><span style="color:#c9d1d9;"> build</span><span style="color:#ffa657;"> --release</span></pre>\n'}
            <blockquote>
                <p><strong>Note:</strong> Building via cargo can generate a decent amount of debug files. 
                    If you want to generate the minimalist binary remove <code>debug = true</code> from <code>Cargo.toml</code>!</p>
            </blockquote>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={4} name="Docker" setSelIndex={setTabIndex} OSIcon={DockerIcon} setSpacer={setSpacerHeight}>
            <h4>Container</h4>
            <p>If you want to use Feather via Docker there currently is the following Container available: 
                <a href="https://hub.docker.com/r/buddyspencer/feather" target="_blank" rel="noopener">buddyspencer/feather</a></p>
        </OSTab>
    </div>
    <div style="height: {spacerHeight}px" class="hidden md:block my-1"></div>
</div>

<svelte:head>
    <title>Download | Feather</title>
</svelte:head>

<style>
    .sanchor{    
        @apply underline cursor-pointer font-medium;
    }
</style>