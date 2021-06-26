<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import type { Plugin } from '$lib/types';

	export let plugin: Plugin;
	let {
		name,
		description,
		versions,
		downloads,
		downloads_recent,
		updated,
		repository,
		homepage
	}: Plugin = plugin;
</script>

<Card>
	<div class="flex flex-wrap justify-between">
		<div class="flex flex-col justify-between">
			<div class="mr-4">
				<div class="flex items-end">
					<a
						href="/plugins/{name}"
						class="text-2xl font-bold hover:text-gray-600 dark:hover:text-gray-400"
					>
						{name}
					</a>
					<ol class="flex">
						{#each versions as version}
							<li class="ml-1 text-sm">{version}</li>
						{/each}
					</ol>
				</div>
				<p>{description}</p>
			</div>
			<ul class="flex space-x-2 mt-2">
				{#if homepage}
					<li>
						<a
							href={homepage}
							class="font-bold border-b-2 border-feather-accent hover:text-gray-600">Homepage</a
						>
					</li>
				{/if}
				{#if repository}
					<li>
						<a
							href={homepage}
							class="font-bold border-b-2 border-feather-accent hover:text-gray-600">Repository</a
						>
					</li>
				{/if}
			</ul>
		</div>
		<div class="flex flex-col justify-center text-lg">
			<div class="my-1">
				<span
					><abbr title="Total numbers of downloads">All-Time:</abbr>
					{downloads.toLocaleString()}</span
				>
			</div>
			<div class="my-1">
				<span
					><abbr title="Downloads in the last 90 days">Recent:</abbr>
					{downloads_recent.toLocaleString()}</span
				>
			</div>
			<div class="my-1">
				<span
					><abbr title="The last time plugin was updated">Updated:</abbr>
					{updated.toLocaleString()} days ago</span
				>
			</div>
		</div>
	</div>
</Card>
