<script context="module">
    export async function preload({ query: { code } }) {
        return { code };
    }
</script>

<script>
    import { goto } from "@sapper/app";
    import { token } from "$stores/local.js";

    export let code;

    const login = async (code) => {
        if (!process.browser) {
            return {};
        } else if ($token.secret) {
            await goto("/me");
        } else if (code) {
            const response = await fetch(
                `process.env.FEATHER_API/me/authorization?code=${code}`
            );
            if (response.status === 200) {
                const { secret } = await response.json();
                $token = { secret };
            }
            await goto("/me");
        } else {
            await goto("process.env.GITHUB_IDENTITY");
        }
    };
</script>

{#await login(code, token)}
    ...
{:then { }}
    ...
{/await}
