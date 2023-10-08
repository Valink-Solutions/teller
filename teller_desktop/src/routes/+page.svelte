<script>
	import { goto } from '$app/navigation';
	import { activeItem, currentDir } from '$lib/stores/navigation';

	import { onMount } from 'svelte';

	onMount(() => {
		let defaultOptions = {
			type: 'world',
			category: 'default',
			path: 'default'
		};
		currentDir.set(defaultOptions);
		activeItem.set(defaultOptions);
		goto('/local/worlds/default/default');
	});
</script>
