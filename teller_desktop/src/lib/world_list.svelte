<script lang="ts">
	import { onMount } from 'svelte';
	import type { WorldItem } from './utils';
	import WorldListItem from './world_list_item.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Icon from '@iconify/svelte';
	import type { CurrentDir } from '../stores';

	let worlds: WorldItem[] = [];

	export let saves_path = '';

	export let currentDir: CurrentDir = { path: 'default', category: null };

	let loading = true;
	let error = false;

	onMount(async () => {
		try {
			let result = await invoke('grab_local_worlds_list', { localSavesPath: saves_path });

			if (Array.isArray(result)) {
				worlds = result as WorldItem[];
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

	$: {
		loading = true;
		error = false;

		invoke('grab_local_worlds_list', { localSavesPath: saves_path })
			.then((result) => {
				if (Array.isArray(result)) {
					worlds = result as WorldItem[];
					loading = false;
				} else {
					console.log(result);
				}
			})
			.catch((err) => {
				console.log(err);
				error = true;
			});
	}
</script>

<div class="flex flex-col justify-start h-full w-full px-2">
	<div class="flex flex-row p-1 w-full h-fit items-center gap-2 align-top">
		<h1 class="border-l-4 pl-2 capitalize border-primary my-2 whitespace-nowrap">
			{currentDir.path}:
		</h1>
		<span class="text-xs underline whitespace-nowrap"
			>{saves_path.slice(0, 25) + '...' + saves_path.slice(-25)}</span
		>
	</div>
	<div class="flex flex-col justify-start items-center w-full h-full gap-3 align-top">
		{#if loading}
			<div class="flex flex-col justify-center items-center h-full w-full gap-3 align-middle">
				<Icon icon="mdi:loading" class="w-16 h-16 animate-spin" />
				<p class="text-lg font-semibold">Loading...</p>
			</div>
		{:else if error}
			<div class="flex flex-col justify-center items-center h-full w-full gap-3 align-middle">
				<h1 class="text-2xl font-bold">Error</h1>
				<p class="text-center">There was an error loading the worlds.</p>
			</div>
		{:else if worlds.length > 0}
			{#each worlds as world}
				<WorldListItem {world} {currentDir} />
			{/each}
		{:else}
			<div class="flex flex-col justify-center items-center h-full w-full gap-3 align-middle">
				<h1 class="text-2xl font-bold">No Worlds Found</h1>
				<p class="text-center">Try adding a world to your saves folder.</p>
			</div>
		{/if}
	</div>
</div>
