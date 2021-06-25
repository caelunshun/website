<script context="module" lang="ts">
	import { API_BASE_URL } from '$lib/env';
	import * as docscache from '$lib/stores/docscache';
	import type { DocResponse } from '$lib/types';
	import type { Load } from '@sveltejs/kit';

	/** @type {import('@sveltejs/kit').Load} */
	export const load: Load = async ({ fetch }) => {
		if (docscache.has('$summary')) {
			return { props: { summaryHtml: docscache.get('$summary') } };
		} else {
			const response_fetch = await fetch(`${API_BASE_URL}/docs/summary`);
			let response: DocResponse = await response_fetch.json();
			let markdown_html = response.html;
			markdown_html = markdown_html.replace(
				/https\:\/\/raw\.githubusercontent\.com\/Defman\/feather\/Docs\/docs\/src\//g,
				'/docs/'
			);
			docscache.set('$summary', markdown_html);
			return { props: { summaryHtml: markdown_html } };
		}
	};
</script>

<script lang="ts">
	import { MenuIcon, XIcon, ArrowLeftIcon } from 'svelte-feather-icons';
	import { docheadings, preferences } from '$lib/stores/local';
	import { browser } from '$app/env';
	import DarkThemeSwitch from '$lib/components/DarkThemeSwitch.svelte';
	import { page } from '$app/stores';

	export let summaryHtml: string;

	let isMobile = false;
	if (browser) {
		let query = window.matchMedia('(max-width: 768px)');
		isMobile = query.matches;
		query.addEventListener('change', (e) => {
			isMobile = e.matches;
		});
	}
	let isAsideShown: boolean = !isMobile;

	page.subscribe(() => {
		if (isMobile) isAsideShown = false;
	});
</script>

<div
	class="flex flex-1 transition-colors duration-300 {$preferences.dark
		? 'bg-gray-900 text-gray-50 dark'
		: ''}"
>
	<aside
		class="summaryaside transform flex bg-green-600 dark:bg-nether-netherrack text-white md:translate-x-0 {isAsideShown
			? ''
			: '-translate-x-full md:hidden'}"
	>
		<div class="flex justify-between items-center py-3 px-6 mb-1 dark:bg-nether-netherrack-dark">
			<a href="/" class="flex items-center">
				<ArrowLeftIcon class="mr-2 h-8 w-8" />
				<h1 class="text-2xl">Go back</h1>
			</a>
			<button class="md:hidden" on:click={() => (isAsideShown = false)}>
				<XIcon class="w-10 h-10 p-1 border border-gray-300 rounded-lg" />
			</button>
		</div>
		<nav class="summary px-12">
			{@html summaryHtml}
		</nav>
	</aside>
	<article class="flex flex-1 flex-col px-8 text-lg overflow-x-scroll md:overflow-x-visible">
		<div class="flex justify-between items-center mt-4">
			<div class="flex z-30 cursor-pointer" on:click={() => (isAsideShown = !isAsideShown)}>
				<MenuIcon
					class="text-white fill-current h-10 w-10 bg-green-600 dark:bg-nether-netherrack rounded-lg p-2 border border-gray-300"
				/>
			</div>
			<div class="flex">
				<DarkThemeSwitch class="border dark:border-gray-500" />
			</div>
		</div>
		<div class="w-full prose xl:prose-xl dark:prose-dark markdown mx-auto md:my-16 my-6">
			<slot />
		</div>
	</article>
	<aside
		class="hidden md:block border-l border-black dark:border-white pl-4 py-4 relative overflow-y-hidden w-1/5"
	>
		<h1 class="text-2xl fixed">Topics</h1>
		<ul class="block mt-8 fixed overflow-y-auto max-h-120 pr-14">
			{#each $docheadings as heading, idx (idx)}
				<li
					class="block text-lg cursor-pointer px-2 my-1 border-l border-blue-500 transition-colors hover:bg-blue-500 duration-300 hover:text-black"
				>
					<a href={'#' + heading.hash}>{heading.name}</a>
				</li>
			{/each}
		</ul>
	</aside>
</div>

<style>
	.summaryaside {
		@apply flex-grow flex-col md:flex-grow-0 z-40 transition md:flex-shrink pb-8 md:static absolute w-screen md:w-auto;
	}
</style>
