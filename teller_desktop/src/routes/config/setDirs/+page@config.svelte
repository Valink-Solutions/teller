<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import DirectoriesList from '$lib/directories_list.svelte';
	import { onMount } from 'svelte';

	onMount(async () => {
		await appWindow.setTitle('Set World Save Directories').catch((err) => {
			console.error(err);
		});
	});
</script>

<div class="p-2">
	<DirectoriesList />
</div>
