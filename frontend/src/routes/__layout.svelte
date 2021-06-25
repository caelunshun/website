<script lang="ts">
	import '../global.css';
	import { token, preferences, achieve } from '$lib/stores/local';
	import { onMount } from 'svelte';
	import { scale, fly } from 'svelte/transition';
	import Logo from '$lib/assets/chicken2.svg?component';
	import DarkThemeSwitch from '$lib/components/DarkThemeSwitch.svelte';
	import { LockIcon, SettingsIcon, MenuIcon, ChevronDownIcon } from 'svelte-feather-icons';
	import Achievement from '$lib/components/Achievement.svelte';

	let isDropdownShown: boolean = false;
	let isAccountDropdownShown: boolean = false;

	let accountdropdownli: HTMLLIElement = null;

	onMount(() => {
		const clicklistener = (event: Event) => {
			if (isAccountDropdownShown && !accountdropdownli.contains(event.target as HTMLElement)) {
				isAccountDropdownShown = false;
			}
		};
		const anchorclicklistener = () => {
			if (isDropdownShown) {
				isDropdownShown = false;
			}
		};
		document.addEventListener('click', clicklistener, false);
		document
			.querySelectorAll('header a[href^="/"]')
			.forEach((anchorelem) =>
				(anchorelem as HTMLAnchorElement).addEventListener('click', anchorclicklistener, false)
			);
	});

	let isEventChannelBlocked = false;

	function toggleAccountDropDown() {
		if (!isEventChannelBlocked) isAccountDropdownShown = !isAccountDropdownShown;
	}

	//Apparently Click is fired 50-100ms after focus
	function blockedEventChannel(e: FocusEvent | MouseEvent) {
		if (e instanceof FocusEvent) {
			isAccountDropdownShown = true;
			isEventChannelBlocked = true;
			setTimeout(() => (isEventChannelBlocked = false), 100);
		} else {
			toggleAccountDropDown();
		}
	}

	let scrollY: number;
</script>

<svelte:window bind:scrollY />

<header
	class="flex {$preferences.dark
		? 'bg-nether-netherrack netherrack'
		: 'bg-green-600'} text-white sticky top-0 z-50 transition duration-500 {scrollY > 0
		? 'shadow-xl'
		: ''}"
	role="heading"
	aria-level={1}
>
	<nav
		class="flex flex-1 flex-col md:flex-row items-center md:h-24 py-4 px-4 container mx-auto relative {isDropdownShown
			? 'max-h-96'
			: 'max-h-20 md:max-h-24'}"
		style="transition: max-height .5s;"
		role="navigation"
		aria-label="Main"
	>
		<div class="flex w-full justify-between">
			<a href="/" class="flex items-center">
				<Logo class="h-8 sm:h-12" />
				<div class="ml-3 text-4xl block"><span>Feather</span></div></a
			>
			<div
				class="flex md:hidden z-50 text-white"
				on:click={() => {
					isDropdownShown = !isDropdownShown;
				}}
			>
				<MenuIcon class="text-white h-10 w-10" />
			</div>
		</div>
		<ul
			class="md:flex w-full md:w-auto items-center md:space-x-6 space-y-4 md:space-y-0 mt-4 md:mt-0 text-normal sm:text-xl font-bold {isDropdownShown
				? ''
				: 'hidden md:block'}"
		>
			<li class="hover:underline"><a href="/association">Association</a></li>
			<li class="hover:underline"><a href="/plugins">Plugins</a></li>
			<li class="hover:underline"><a href="/docs">Docs</a></li>
			<li class="hover:underline"><a href="/faq">FAQ</a></li>
			<hr class="md:hidden" />
			<!-- where should we put the link to the main git repo -->
			<!-- <li><a href="https://github.com/feather-rs/feather"><GithubMark class="h-8 sm:h-12 fill-current text-feather-light hover:text-white" /></a></li> -->
			<!-- Should be logged in users avatar -->
			{#if $token.secret}
				<li class="noaflex flex md:hidden">
					<a href="/me">
						<SettingsIcon class="h-6 w-6" />
						<span class="mx-2 my-auto">Account</span>
					</a>
				</li>
				<li class="noaflex font-normal flex md:hidden">
					<a href="/me/logout">
						<LockIcon class="h-6 w-6" />
						<span class="mx-2 my-auto">Logout</span></a
					>
				</li>
				<li
					bind:this={accountdropdownli}
					class="noaflex hidden md:flex cursor-pointer hover:underline relative"
					tabindex={0}
					on:keyup={(e) => {
						if (e.code === 'Space') toggleAccountDropDown();
					}}
					on:focus={blockedEventChannel}
					on:click={blockedEventChannel}
				>
					<span class="mx-2 my-auto">Account</span>
					<div class="my-auto">
						<ChevronDownIcon class="text-white h-8 w-8" />
					</div>
					{#if isAccountDropdownShown}
						<div
							in:scale={{ duration: 100, start: 0.95 }}
							out:scale={{ duration: 75, start: 0.95 }}
							class="origin-top-right absolute right-0 w-48 mt-10 bg-gray-800 rounded-lg border border-white"
						>
							<a
								href="/me"
								class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-t-lg"
								>Profile</a
							>
							<hr />
							<a
								href="/me/logout"
								class="block px-4 py-2 hover:bg-green-500 hover:text-green-100 rounded-b-lg"
								on:keydown={(e) => {
									if (e.code === 'Tab') toggleAccountDropDown();
								}}>Logout</a
							>
						</div>
					{/if}
				</li>
			{:else}
				<!-- How should the login button look? -->
				<li class="hover:underline">
					<a href="/me/login" class="flex items-center">
						<LockIcon class="h-6 w-6" />
						<span class="mx-2 my-auto whitespace-nowrap">Login (GitHub)</span></a
					>
				</li>
			{/if}
		</ul>
	</nav>
</header>
{#if $achieve.isAchievementShown}
	<div class="fixed right-4 top-28 z-50" transition:fly={{ x: 300, duration: 2000 }}>
		<Achievement title="We Need To Go Deeper!" message="Activate Dark/Nether Theme!" />
	</div>
{/if}
<main
	class="flex flex-col flex-1 relative transition-colors duration-200 {$preferences.dark
		? 'bg-gray-900 text-gray-50 dark'
		: ''}"
	role="main"
>
	<slot />
</main>
<footer
	class="{$preferences.dark
		? 'bg-nether-netherrack-dark netherrack'
		: 'bg-green-600'} text-white sm:flex sm:items-center justify-center transition-colors duration-500"
>
	<bold class="text-lg my-2 block sm:flex text-center">
		Made with <span class="text-red-600 sm:mx-1">&#10084;</span> by the Feather Association.
		<a href="credits" class="underline sm:mx-1">Credits</a>
	</bold>
	<DarkThemeSwitch class="mx-auto sm:mx-2 my-2 sm:my-0" />
</footer>

<style>
	/* li > a {
        @apply tracking-wide md:tracking-normal mx-auto hover:underline focus:text-blue-400 transition-colors;
    }
    li > a:not(.noaflex) {
        @apply flex;
    }
    li:not(.noaflex) {
        @apply flex;
    } */

	.netherrack {
		background-image: url('/netherrack_overlay.webp');
		background-size: 64px;
		background-repeat: repeat;
	}
</style>
