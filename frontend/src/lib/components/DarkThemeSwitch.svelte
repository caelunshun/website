<script lang="ts">
	import { preferences } from '$lib/stores/local';

	import ClockOutline from '$lib/assets/minecraft/clock_outline.svg?component';

	let extraClazz: string;
	export { extraClazz as class };

	function toggleDarkMode() {
		$preferences.dark = !$preferences.dark;
	}

	function darkModeSwitchKeyPressHandler(event: KeyboardEvent) {
		if (event.code === 'Space') toggleDarkMode();
	}
</script>

<div
	class="w-16 h-8 flex items-center {$preferences.dark
		? 'bg-gray-900'
		: 'bg-gray-300'} rounded-full cursor-pointer {extraClazz}"
	role="switch"
	aria-checked={$preferences.dark}
	aria-label="Toggle Dark Mode"
	on:keyup={darkModeSwitchKeyPressHandler}
	on:click={toggleDarkMode}
	tabindex={0}
>
	<div
		class="w-8 h-8 rounded-full shadow-md transform my-auto {$preferences.dark
			? 'translate-x-8'
			: ''} duration-300 ease-in-out flex items-center justify-center"
	>
		<div class="relative w-8 h-8">
			<ClockOutline class="absolute" />
			<div class="mc-clock-after w-8 h-8 {$preferences.dark ? 'clockdark' : ''}" />
		</div>
	</div>
</div>

<style>
	.mc-clock-after {
		position: absolute;
		z-index: -1;
		background-image: url(data:image/webp;base64,UklGRn4AAABXRUJQVlA4THEAAAAvD8ADEDegGJIkZjkS1fm/TiNpKGjbhimO7oEa/otkCtpGgU7IiyKsIBgx8x8A/GOC+k7lI8fUAptIklW99ynn+NcDAV/EN0A5r38PRxYi+j8Bzn4BQEOmm1t6rhWmmwPIHEIIBdkytx7cKWyZC4GYAQA=);
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		transition: transform 0.3s;
	}
	.clockdark {
		transform: rotate(180deg);
	}
</style>
