<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { currentDir } from '$lib/stores';
	import WorldList from '$lib/world_list.svelte';
	import Icon from '@iconify/svelte';
	import type { WorldItem } from '$lib/utils';

	let world_path: string = '';
	let worlds: WorldItem[] = [];

	let loading = true;
	let error = false;

	$: {
		currentDir.subscribe(async (value) => {
			loading = true;
			try {
				const result = await invoke('plugin:config|get_folder_path', {
					dirName: value.path,
					category: value.category
				});
				if (result) {
					world_path = result as string;
					let worldResult = await invoke('plugin:folder_handler|grab_local_worlds_list', {
						localSavesPath: world_path
					});
					if (Array.isArray(worldResult)) {
						worlds = worldResult;
					} else {
						console.log(worldResult);
					}
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
		const result = await invoke('plugin:config|get_folder_path', {
			dirName: $currentDir.path,
			category: $currentDir.category
		});
		if (result) {
			world_path = result as string;
			let worldResult = await invoke('plugin:folder_handler|grab_local_worlds_list', {
				localSavesPath: world_path
			});
			if (Array.isArray(worldResult)) {
				worlds = worldResult;
			} else {
				console.log(worldResult);
			}
		} else {
			console.log(result);
		}
	});

	// This had to be done to fix path issues caused by how the functions return data
	async function openInstanceFolder(path: string) {
		await invoke('plugin:folder_handler|open_path_in_explorer', {
			path: path
		});
	}
</script>

<div class="flex flex-col h-full w-full">
	<div class="flex flex-row w-full h-fit items-center gap-2 align-top px-2">
		<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
		<h1
			class="border-l-4 pl-2 w-fit max-w-[240px] capitalize border-primary my-2 whitespace-nowrap text-elipsis overflow-hidden overflow-ellipsis"
		>
			{$currentDir.path}
		</h1>
		<button
			on:click={() => openInstanceFolder(world_path)}
			class="transition-opacity group flex flex-row items-center gap-1 text-xs underline whitespace-nowrap"
		>
			<span class="opacity-70">{world_path.slice(0, 25) + '...' + world_path.slice(-25)}</span>
			<Icon icon="mdi:folder-open-outline" class="opacity-0 group-hover:opacity-70" />
		</button>
	</div>
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
		<div class="flex px-2">
			<WorldList {worlds} on:visible currentDir={$currentDir} />
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:alert" class="w-16 h-16" />
			<p class="text-lg font-semibold">No data available</p>
		</div>
	{/if}
</div>
