<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	// import DirectoriesList from '$lib/directories_list.svelte';
	import { onMount } from 'svelte';
	import NewDirectoriesList from '$lib/new_directories_list.svelte';

	onMount(async () => {
		appWindow.setResizable(false).catch((err) => {
			console.error(err);
		});

		await appWindow.setTitle('Set World Save Directories').catch((err) => {
			console.error(err);
		});
	});
</script>

<NewDirectoriesList />
