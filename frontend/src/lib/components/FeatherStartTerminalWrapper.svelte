<script lang="ts">
    import Terminal from "./Terminal.svelte";
    import type { Output } from "../types";
    function rn() {
        let date = new Date();
        return `${date.getFullYear()}-${
            (date.getMonth() + 1).toString().padStart(2, "0")
        }-${
            date.getDate().toString().padStart(2, "0")
        } ${
            date.getHours().toString().padStart(2, "0")
        }:${
            date.getMinutes().toString().padStart(2, "0")
        }:${
            date.getSeconds().toString().padStart(2, "0")
        },${
            date.getMilliseconds().toString().padStart(3, "0")
        } `;
    }
    let outputs: Output[] = [
        {
            timeout: 1000,
            message: () => "feather-test$ ",
            newline: true,
            noslow: true,
            caret: true
        },
        {
            timeout: 2000,
            message: () => "./feather-server",
            newline: false,
            typewriter: true,
            noslow: true
        },
        {
            timeout: 0,
            message: () => rn() + "INFO  [feather_server] Loaded config",
            newline: true
        },
        {
            timeout: 0.38,
            message: () => rn() + "INFO  [feather_server] Creating server",
            newline: true
        },
        {
            timeout: 0.3,
            message: () => rn() + "INFO  [feather_server] Server is listening on 0.0.0.0:25565",
            newline: true
        },
        {
            timeout: 0.1,
            message: () =>
                rn() + "INFO  [feather_common::region_worker] Chunk worker started",
            newline: true
        },
        {
            timeout: 6700,
            message: () => "^C",
            newline: true,
            noslow: true
        },
        {
            timeout: 100,
            message: () => "feather-test$ ",
            newline: true,
            noslow: true,
            caret: true
        },
        {
            timeout: 700,
            message: () => "clear",
            newline: false,
            typewriter: true,
            noslow: true
        }
    ];
</script>

<Terminal {outputs} />
