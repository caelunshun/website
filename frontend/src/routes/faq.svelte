<script lang="ts">
    interface Question {
        question: string;
        answer: string;
    }

    let questions: Question[] = [
        {
            question: "Is Feather ready for production?",
            answer: "No! Feather is in big alpha! Many core features still are in work!"
        },
        {
            question: "What versions does Feather support?",
            answer: "As of now Feather only supports 1.16 but will probably switch to 1.17 soon."
        },
        {
            question: "Does Feather support plugins?",
            answer:
                "Yes. Feather allows developers to write plugins with basically any language that can be turned into " +
                '<a href="https://webassembly.org/" target="_blank" rel="noopener">Wasm</a>.'
        },
        {
            question: "Why are plugins written in Wasm and not Lua (etc.)?",
            answer:
                "The reason that plugins are written in Wasm is that it allows development in many languages, " +
                "is pretty performant <em>(up to 98% in comparison to natively integrated)</em> and doesn't increase Feathers binary size."
        },
        {
            question: "Where can I submit bugs/unusual behavior?",
            answer:
                'Bugs (etc.) can be submitted either on <a href="https://github.com/feather-rs/feather/issues" target="_blank" rel="noopener">GitHub</a> ' +
                'or on our <a href="https://discordapp.com/invite/4eYmK69" target="_blank" rel="noopener">Discord</a>.'
        },
        {
            question: "Where can I submit problems with the website?",
            answer:
                "Problems with the website of Feather can be submitted on " +
                '<a href="https://github.com/feather-rs/website/issues" target="_blank" rel="noopener">its GitHub repository</a>.'
        }
    ];
</script>

<div class="min-h-full container mx-auto py-8 px-2">
    <h2 class="text-4xl font-bold my-4">FAQ &mdash; Feather</h2>
    <hr class="mb-4" />
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 typography">
        {#each questions as question, idx (idx)}
            <div>
                <span class="text-xl font-bold">{question.question}</span>
                <p class="mt-2 text-lg dark:text-gray-300 text-gray-500">{@html question.answer}</p>
            </div>
        {/each}
    </div>
</div>

<svelte:head>
    <title>FAQ | Feather</title>
</svelte:head>

<style>
</style>
