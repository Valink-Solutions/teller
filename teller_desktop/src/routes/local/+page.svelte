<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { currentDir } from '../../stores';
	import WorldList from '$lib/world_list.svelte';
	import Icon from '@iconify/svelte';

	let world_path: string;

	let loading = true;
	let error = false;

	$: {
		currentDir.subscribe(async (value) => {
			try {
				const result = await invoke('get_folder_path', {
					dirName: value.path,
					category: value.category
				});
				if (result) {
					world_path = result as string;
				} else {
					console.log(result);
				}
			} catch (err) {
				console.log(err);
				error = true;
			} finally {
				loading = false;
			}
		});
	}

	onMount(async () => {
		const result = await invoke('get_folder_path', {
			dirName: $currentDir.path,
			category: $currentDir.category
		});
		if (result) {
			world_path = result as string;
		} else {
			console.log(result);
		}
	});
</script>

{#if loading}
	<div class="flex flex-col items-center justify-center m-auto w-full h-full">
		<Icon icon="mdi:loading" class="w-16 h-16 animate-spin" />
		<p class="text-lg font-semibold">Loading...</p>
	</div>
{:else if error}
	<div class="flex flex-col items-center justify-center w-full h-full">
		<Icon icon="mdi:error" class="w-16 h-16" />
		<p class="text-lg font-semibold">Error loading data</p>
	</div>
{:else if world_path}
	<WorldList on:visible saves_path={world_path} currentDir={$currentDir} />
{:else}
	<div class="flex flex-col items-center justify-center w-full h-full">
		<Icon icon="mdi:alert" class="w-16 h-16" />
		<p class="text-lg font-semibold">No data available</p>
	</div>
{/if}
