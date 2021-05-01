<script>
    import Card from "$components/Card.svelte"
    import { CheckIcon, XIcon } from "svelte-feather-icons"
    let invites = [
        {
            plugin_name: "world-edit"
        }
    ]
</script>

<div class="container mx-auto px-4 py-8">
    <h1 class="text-4xl font-bold">Pending Owner Invites</h1>
    <ul>
        {#each invites as invite}
            <li class="mt-4">
                <Card class="flex justify-between">
                    <h2 class="text-xl mr-4">{invite.plugin_name}</h2>
                    <div class="flex">
                        <button title="Accept the Invite" class="mr-4" tabindex={2}><CheckIcon class="h-8 w-8" /></button>
                        <button title="Decline the Invite" tabindex={2}><XIcon class="h-8 w-8" /></button>
                    </div>
                </Card>
            </li>
        {/each}
    </ul>
</div>