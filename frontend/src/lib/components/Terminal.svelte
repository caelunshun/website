<script lang="ts">
    import {onMount} from "svelte";
    import type {Output, TerminalOutput} from "$lib/types";
    import {browser} from "$app/env";

    export let outputs: Output[] = [];
    let lines: TerminalOutput[] = [];
    let ismultiplierselshown = false;
    let multiplier = 1;
    let currentmp = multiplier;

    const awaitfocus = async () => {
        while(document.visibilityState === 'hidden') {
            await timeout(100);
        }
    }

    const interval = () => {
        let timeout = 15000;
        switch(multiplier) {
            case 1000:
                timeout = 18000;
                break;
            case 2500:
                timeout = 18000;
                break;
        }
        setTimeout(() => {
            lines = [];
            awaitfocus().then(() => {
                createTerm();
                interval();
            });
        }, timeout)
    }

    onMount(() => {
        if(browser) {
            createTerm();
            interval();
        }
    });
    async function createTerm() {
        currentmp = multiplier;
        let curcallbackstack: number = 0;
        for(let output of outputs) {
            let timeout = output.timeout;
            if(!output.noslow) {
                timeout*=currentmp;
            }
            setTimeout(() => {
                if(output.newline) {
                    lines = [...lines, {message: output.message(), caret: output.caret === true}];
                } else {
                    if(output.typewriter) {
                        const msg = output.message();
                        let curcallbackstack2 = 0;
                        for(let char of Array.from(msg)) {
                            setTimeout(() => {
                                let clines = lines;
                                clines[clines.length-1].message+=char;
                                lines = clines;
                            }, curcallbackstack2+100);
                            curcallbackstack2+=100;
                        }
                    } else {
                        let clines = lines;
                        clines[clines.length-1].message+=output.message();
                        lines = clines;
                    }
                }
            }, curcallbackstack+timeout);
            curcallbackstack+=(timeout + (output.typewriter ? output.message().length*105+1000 : 0));
        }
    }

    function timeout(ms: number) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
</script>
    
<div id="term" class="w-11/12 lg:w-7/12 mx-auto font-mono" on:mouseenter={() => {ismultiplierselshown = true}} on:mouseleave={() => {ismultiplierselshown = false}}>
    <div id="termbar" class="flex w-full h-6 rounded-t-lg text-gray-700">
        <strong class="m-auto">feather-test &mdash; -bash</strong>
    </div>
    <div id="termscreen" class="w-full mx-auto p-0.5 rounded-b-lg overflow-y-auto relative" style="height: 30rem;">
        <ul class="flex flex-col">
            {#each lines as message, idx (idx)}
                <li class="termline">
                    <span class:caret={message.caret === true}>{message.message}</span>
                </li>
            {/each}
        </ul>
        <div id="termsel" class="{ismultiplierselshown ? "opacity-100" : "opacity-0"} border border-gray-200 transform -translate-y-1/2 -translate-x-1/2 w-10/12 transition-opacity duration-1000 rounded-xl absolute -bottom-4 left-1/2">
            <strong class="text-center mt-2 block" style="color: #fff;">Slowmotion (decrease the speed of the "process" output)</strong>
            <div class="border border-gray-200 flex m-4 rounded-lg">
                <button class="flex flex-grow py-2" style="color: #fff;" on:focus={() => multiplier = 1} selected={multiplier === 1}><b class="m-auto">1x</b></button>
                <button class="flex flex-grow py-2" style="color: #fff;" on:focus={() => multiplier = 100} selected={multiplier === 100}><b class="m-auto">100x</b></button>
                <button class="flex flex-grow py-2" style="color: #fff;" on:focus={() => multiplier = 1000} selected={multiplier === 1000}><b class="m-auto">1000x</b></button>
                <button class="flex flex-grow py-2" style="color: #fff;" on:focus={() => multiplier = 2500} selected={multiplier === 2500}><b class="m-auto">2500x</b></button>
            </div>
        </div>
    </div>
    
</div>

<style>
    #termbar {
        background-color: #DAD9D9;
    }
    #termscreen {
        background-color: #33485E;
    }
    .termline {
        color: #fff;
        font-size: 15px;
        text-align: left;
        position: static;
    }
    .termline:last-child .caret {
        border-right: .15em solid #fff;
        animation: blink-caret .75s step-end infinite;
    }
    @keyframes blink-caret {
        from, to { border-color: #fff }
        50% { border-color: transparent; }
    }
    #termsel button {
        border-right: 1px solid #fff;
    }
    #termsel button:focus {
        outline: none;
    }
    #termsel button[selected="true"]{
        background-color: rgba(255, 255, 255, 0.4);
    }
    #termsel button:first-child {
        padding-left: 0.5rem;
    }
    #termsel button:last-child {
        padding-right: 0.5rem;
        border-right: none;
    }
</style>
