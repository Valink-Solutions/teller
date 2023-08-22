<script lang="ts">
	import { onMount } from 'svelte';
	import type { WorldItem } from './utils';
	import WorldListItem from './world_list_item.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let worlds: WorldItem[] = [];

	export let saves_path = '';

	export let currentDir = '';

	onMount(() => {
		invoke('grab_local_worlds_list', { localSavesPath: saves_path }).then((result) => {
			if (Array.isArray(result)) {
				worlds = result as WorldItem[];
			} else {
				console.log(result);
			}
		});
	});

	$: {
		worlds = [];

		invoke('grab_local_worlds_list', { localSavesPath: saves_path }).then((result) => {
			if (Array.isArray(result)) {
				worlds = result as WorldItem[];
			} else {
				console.log(result);
			}
		});
	}

	console.log(worlds);
</script>

<div class="flex flex-col justify-start h-full w-full px-2">
	<div class="flex flex-row p-1 w-full h-fit items-center gap-2 align-top">
		<h1 class="border-l-4 pl-2 capitalize border-primary my-2 whitespace-nowrap">
			{currentDir}:
		</h1>
		<span class="text-xs underline whitespace-nowrap"
			>{saves_path.slice(0, 25) + '...' + saves_path.slice(-25)}</span
		>
	</div>
	<div class="flex flex-col justify-start items-center w-full h-full gap-3 align-top">
		{#if worlds.length > 0}
			{#each worlds as world}
				<WorldListItem {world} />
			{/each}
		{:else}
			<div class="flex flex-col justify-center items-center h-full w-full gap-3 align-middle">
				<h1 class="text-2xl font-bold">No Worlds Found</h1>
				<p class="text-center">Try adding a world to your saves folder.</p>
			</div>
		{/if}
	</div>
</div>
