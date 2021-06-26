<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/env';

	if (browser) {
		onMount(async () => {
			await goto('/docs/introduction');
		});
	}
</script>
