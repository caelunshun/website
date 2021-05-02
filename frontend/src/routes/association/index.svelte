<script lang="ts">
    import {preferences} from "$stores/local";
</script>

<svelte:head>
    <title>Association | Feather</title>
</svelte:head>

<section class="dark:bg-gray-800 bg-blue-50 typography">
    <div class="container mx-auto flex flex-col py-16">
        <h1 class="text-6xl m-4 sm:m-8">The Feather Association</h1>
        <div class="flex flex-wrap lg:flex-nowrap">
            <p class="text-lg m-4 sm:m-8">
                The purpose of the Feather Minecraft Association is to facilitate the development of Feather, a Minecraft server implementation written in Rust, and related services like the plugin registry. Membership fee and donations go towards this goal and are used to maintain infrastructures like servers and domain name.
                <br/>
                <br/>
                The association is lead by the elected Board of Directors, they are responsible for daily administrative work. They are elected once a year by the present members at the General Assembly which is held once a year and by the end of ____.
                <br/>
                <br/>
                The membership fee is 10 EUR and is valid from the month in which the membership is paid and the following 11 months.
            </p>
        </div>
    </div>
</section>
<section class="typography">
    <div class="container mx-auto flex flex-col py-16">
        <h3 class="text-6xl m-4 sm:m-8">Further about the association</h3>
        <div class="flex flex-wrap lg:flex-nowrap">
            <div class="m-4 sm:m-8">
                <h4>Budget</h4>
                <p class="mt-4 text-lg">
                    To learn more about the income and general budget of Fether click <a href="/association/budget" >here</a>!
                </p>
            </div>
            <div class="m-4 sm:m-8">
                <h4>Articles of Association</h4>
                <p class="mt-4 text-lg">
                    To learn more about the articles of association of Fether click <a href="/association/articles-of-association" >here</a>!
                </p>
            </div>
        </div>
    </div>
</section>