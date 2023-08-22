<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { currentDir } from '../../stores';
	import WorldList from '$lib/world_list.svelte';

	let world_path: string;

	$: {
		currentDir.subscribe((value) => {
			invoke('get_folder_path', { dirName: value }).then((result) => {
				if (result) {
					world_path = result as string;
				} else {
					console.log(result);
				}
			});
		});
	}

	onMount(() => {
		invoke('get_folder_path', { dirName: $currentDir }).then((result) => {
			if (result) {
				world_path = result as string;
			} else {
				console.log(result);
			}
		});
	});
</script>

<WorldList on:visible saves_path={world_path} currentDir={$currentDir} />
