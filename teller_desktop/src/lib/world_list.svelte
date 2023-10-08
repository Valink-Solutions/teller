<script lang="ts">
	import type { WorldItem } from './types/worlds';
	import WorldListItem from './world_list_item.svelte';
	import type { CurrentDir } from './types/navigation';

	export let worlds: WorldItem[] = [];
	export let currentDir: CurrentDir = { path: 'default', category: null };
</script>

<div class="flex flex-col justify-start min-h-full h-full w-full">
	<div class="flex flex-col justify-start items-center min-h-full w-full h-full gap-3 align-top">
		{#if worlds.length > 0}
			{#each worlds as world}
				<WorldListItem {world} {currentDir} />
			{/each}
		{:else}
			<div
				class="flex flex-col justify-center items-center min-h-full h-full w-full gap-3 align-middle"
			>
				<h1 class="text-2xl font-bold">No Worlds Found</h1>
				<p class="text-center">Try adding a world to your saves folder.</p>
			</div>
		{/if}
	</div>
</div>
