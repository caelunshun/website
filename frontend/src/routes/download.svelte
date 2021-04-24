<script lang="ts">
    import AppleIcon from '$assets/os/apple.svg';
    import WindowsIcon from '$assets/os/windows.svg';
    import LinuxIcon from '$assets/os/linux.svg';
    import DockerIcon from '$assets/os/docker.svg';
    import {PackageIcon} from 'svelte-feather-icons';

    import OSTab from "$components/download/SystemTab.svelte";

    let isMobile = false;

    let selectedTabIndex = -1;
    if(process.browser) {
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
</script>

<div class="container mx-auto">
    <h2 class="text-4xl font-bold my-4 mx-4 md:mx-0">Download</h2>
    <hr class="w-10/12 mx-auto md:w-full">
    <h3 class="text-3xl font-semibold my-4 mx-4 md:mx-0">Platforms</h3>
    <div class="md:relative md:flex flex-grow md:border-b border-gray-500 text-gray-500 w-full">
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={0} name="Linux" setSelIndex={setTabIndex} OSIcon={LinuxIcon} setSpacer={setSpacerHeight}>
            <p>Download for Linux</p>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={1} name="MacOS" setSelIndex={setTabIndex} OSIcon={AppleIcon} setSpacer={setSpacerHeight}>
            <p>Download for MacOS</p>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={2} name="Windows" setSelIndex={setTabIndex} OSIcon={WindowsIcon} setSpacer={setSpacerHeight}>
            <p>Download for Windows</p>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={3} name="From source" setSelIndex={setTabIndex} OSIcon={PackageIcon} fill={false} setSpacer={setSpacerHeight}>
            <h4>Prerequirements</h4>
            <ul>
                <li><a href="https://rustup.rs/">Rust</a> (latest toolchain version)</li>
            </ul>
            <h4>Downloading and Installing</h4>
            <p>To download and install Feather from source do the following:</p>
            <ol>
                <li>Download Feathers source code from <a href="https://github.com/feather-rs/feather">here</a></li>
                <li>Go into the project directory and run <code>cargo build --release</code></li>
                <li>Wait for the build to finish. The finished version should be in <code>target/release/</code></li>
                <li>To start the server run <code>feather-server(.exe)</code></li>
            </ol>
            <h5>Script in one: </h5>
            <!--Idk why but it works this way but not if it is added in manually (maybe because svelte compiles it to rough)-->
            {@html '<pre style="background-color:#2b303b;">\n' + 
                '<span style="color:#8fa1b3;">git</span><span style="color:#c0c5ce;"> clone https://github.com/feather-rs/feather\n'+
                '</span><span style="color:#96b5b4;">cd</span><span style="color:#c0c5ce;"> feather\n'+
                '</span><span style="color:#8fa1b3;">cargo</span><span style="color:#c0c5ce;"> build --release</span></pre>\n'}
            <blockquote>
                <p><strong>Note:</strong> Building via cargo can generate a decent amount of debug files. If you want to generate the minimalist binary you need to compile with rustc!</p>
            </blockquote>
        </OSTab>
        <OSTab curSelIndex={selectedTabIndex} desSelIndex={4} name="Docker" setSelIndex={setTabIndex} OSIcon={DockerIcon} setSpacer={setSpacerHeight}>
            <p>If you want to use Feather via Docker there currently is the following Container available: <a href="https://hub.docker.com/r/buddyspencer/feather">buddyspencer/feather</a></p>
        </OSTab>
    </div>
    <div style="height: {spacerHeight}px" class="hidden md:block my-1"></div>
</div>

<svelte:head>
    <title>Download | Feather</title>
</svelte:head>