<script>
    import timeago from "timeago-simple";
    import Card from "components/card.svelte"    

    let new_token_name = "";

    async function fetch_user() {
        return {
            id: 0,
            name: "Jacob Emil Ulvedal Rosborg",
            github_account: "Defman",
            emails: [{ email: "jacob@rosborg.dk", verified: false }],
            tokens: [
                { name: "macbook", use_count: 4, timestamp: 0 },
                {
                    name: "windows",
                    use_count: 0,
                    timestamp: 0,
                    token: "asdjkasdhASduhaj",
                },
            ],
        };
    }

    async function revoke_token(tokens, token_name) {
        tokens = tokens.filter(({ name }) => name !== token_name);
    }

    async function create_new_token(tokens, token_name) {
        tokens = [
            {
                name: token_name,
                use_count: 0,
                timestamp: 0,
                token: "adsjhasdhjuiq",
            },
            ...tokens,
        ];
    }

    let me = fetch_user();
</script>
<div class="container mx-auto flex flex-col px-4 my-8">
    <h1 class="text-4xl font-bold mt-8">Account settings</h1>
        {#await me}
            Loading...
        {:then { id, name, github_account, emails = [], tokens = [] } } 
            <div class="flex flex-col">
                <div>
                    <h2 class="text-xl font-bold mt-8">Profile Information</h2>
                    <Card class="mt-4">
                        <div
                            class="flex items-center"
                        >
                            <img
                                src="https://avatars.githubusercontent.com/u/1496019?v=4&s=170"
                                alt={name}
                                class="h-16 w-16 mr-4"
                            />
                            <ul>
                                <li>
                                    <span class="font-bold">Name:</span> <span>{name}</span>
                                </li>
                                <li>
                                    <span class="font-bold">Github Account:</span>
                                    <span>{github_account}</span>
                                </li>
                            </ul>
                        </div>
                    </Card>
                </div>
                <div>
                    <h2 class="text-2xl font-bold mt-8">User Email</h2>
                    <ul class="mt-4">
                        {#each emails as { email, verified, edited_email = "" }}
                            <li>
                                <Card class="flex justify-between">
                                    <div class="flex flex-1 mr-4 items-center h-12">
                                        {#if edited_email}
                                            <input
                                                type="text"
                                                bind:value={edited_email}
                                                class="flex flex-1 px-4 py-2 border border-feather-light box-border"
                                            />
                                        {:else}
                                            <div class="px-4 py-2">
                                                <span>{email}</span>
                                                {#if verified}
                                                    <span class="text-green-700 ml-4"
                                                        >Verified!</span
                                                    >
                                                {:else}
                                                    <span class="text-red-700 ml-4"
                                                        >Not verified!</span
                                                    >
                                                {/if}
                                            </div>
                                        {/if}
                                    </div>
                                    <div class="flex items-center">
                                        {#if edited_email}
                                            <button
                                                on:click={() => {
                                                    email = edited_email;
                                                    edited_email = "";
                                                }}
                                                class="mr-4 bg-feather-dark text-white px-4 py-2 hover:text-feather-light font-bold"
                                            >
                                                Save
                                            </button>
                                            <button
                                                on:click={() => {
                                                    edited_email = "";
                                                }}
                                                class="bg-feather-dark text-white px-4 py-2 hover:text-feather-light font-bold"
                                            >
                                                Cancel
                                            </button>
                                        {:else}
                                            <button
                                                on:click={() => {
                                                    edited_email = email;
                                                }}
                                                class="bg-feather-dark text-white px-4 py-2 hover:text-feather-light font-bold"
                                            >
                                                Edit
                                            </button>
                                        {/if}
                                    </div>
                                </Card>
                            </li>
                        {/each}
                    </ul>
                </div>
                <div>
                    <h2 class="text-2xl font-bold mt-8">
                        Email Notification Preferences
                    </h2>
                </div>
                <div>
                    <h2 class="text-2xl font-bold mt-8">API Access</h2>
                    <p>
                        If you want to use plugin commands from the command line, you
                        will need to login with <code>cargo quill login (token)</code> using
                        one of the tokens listed below.
                    </p>
                    <p>
                        When working in shared environments, supplying the token on the
                        command line could expose it to prying eyes. To avoid this,
                        enter <code>cargo quill login</code> and supply your token when prompted.
                    </p>
                    <h3 class="text-xl mt-4">Tokens</h3>
                    <ul>
                        <li class="mt-4">
                            <Card>
                                <div class="flex justify-between">
                                    <input
                                        type="text"
                                        bind:value={new_token_name}
                                        class="flex flex-1 px-4 py-2 border border-feather-light box-border mr-4"
                                        placeholder="New token name"
                                    />
                                    <button
                                        on:click={() => {
                                            create_new_token(tokens, new_token_name);
                                            new_token_name = undefined;
                                        }}
                                        class="bg-feather-dark text-white px-4 py-2 hover:text-feather-light font-bold"
                                    >
                                        Create
                                    </button>
                                </div>
                            </Card>
                        </li>
                        {#each tokens as { name, timestamp, use_count, token }}
                            <li class="mt-4">
                                <Card>
                                    <div class="flex justify-between">
                                        <span class="text-lg font-bold">{name}</span>
                                        <div class="flex">
                                            <div
                                                class="flex flex-col text-sm font-light mr-4"
                                            >
                                                <span
                                                    >Created {timeago.simple(
                                                        new Date(timestamp)
                                                    )}</span
                                                >
                                                {#if use_count > 0}
                                                    <span>Used {use_count} times</span>
                                                {:else}
                                                    <span>Never used </span>
                                                {/if}
                                            </div>
                                            <button
                                                on:click={() => revoke_token(tokens, name)}
                                                class="bg-feather-dark text-white px-4 py-2 hover:text-feather-light font-bold"
                                            >
                                                Revoke
                                            </button>
                                        </div>
                                    </div>
                                    {#if token}
                                        <p>
                                            Please record this token somewhere, you cannot
                                            retrieve its value again. For use on the command
                                            line you can save it to ~/.cargo/quill with:
                                        </p>
                                        <div class="border-2 px-4 py-2 mt-4">
                                            <code>cargo quill login {token}</code>
                                        </div>
                                    {/if}
                                </Card>
                            </li>
                        {/each}
                    </ul>
                </div>
            </div>
        {:catch error}
            <p>Error! {error}</p>
        {/await}
    
</div>
