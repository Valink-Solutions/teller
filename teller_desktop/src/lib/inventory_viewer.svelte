<script lang="ts">
	export let items: { slot: Number; id: String; count: Number; tag: Object }[] = [];

	// Process the items array
	items = items.map((item) => {
		// Check if the item is a bedrock item
		if (item.id === 'minecraft:planks' || item.id === 'minecraft:sapling') {
			// Modify the id based on the tag.Block.states object
			item.id = `minecraft:${
				// @ts-ignore
				item.tag.Block.states.wood_type || item.tag.Block.states.sapling_type
			}_${item.id.split(':')[1]}`;
		}
		return item;
	});
</script>

{#if items}
	<div class="grid grid-cols-9 mx-auto">
		{#each Array(27).fill(null) as _, i}
			<div class="border border-gray-300 p-2 w-16 h-16 relative">
				{#if items.find((item) => item.slot === i + 9 && item.id !== '' && item.count !== 0)}
					{#each items.filter((item) => item.slot === i + 9 && item.id !== '' && item.count !== 0) as foundItem}
						<!-- <span class="absolute top-1 left-2">{foundItem.slot}</span> -->
						<img
							class="w-full h-full"
							src={`https://yamapi.chunkvault.com/images/items/${foundItem.id}`}
							alt={foundItem.id.replace('minecraft:', '')}
						/>
						<span class="absolute bottom-1 right-2 text-white drop-shadow font-bold"
							>{foundItem.count}</span
						>
					{/each}
				{/if}
			</div>
		{/each}
	</div>
	<div class="grid grid-cols-9 mx-auto">
		{#each Array(9).fill(null) as _, i}
			<div class="border border-gray-300 p-2 w-16 h-16 relative">
				{#if items.find((item) => item.slot === i && item.id !== '' && item.count !== 0)}
					{#each items.filter((item) => item.slot === i && item.id !== '' && item.count !== 0) as foundItem}
						<!-- <span class="absolute top-1 left-2">{foundItem.slot}</span> -->
						<img
							class="w-full h-full"
							src={`https://yamapi.chunkvault.com/images/items/${foundItem.id}`}
							alt={foundItem.id.replace('minecraft:', '')}
						/>
						<span class="absolute bottom-1 right-2 text-white drop-shadow font-bold"
							>{foundItem.count}</span
						>
					{/each}
				{/if}
			</div>
		{/each}
	</div>
	<div class="grid grid-cols-4 mt-4 mx-auto">
		{#each [100, 101, 102, 103] as slot}
			<div class="border border-gray-300 p-2 w-16 h-16 relative">
				{#if items.find((item) => item.slot === slot && item.id !== '' && item.count !== 0)}
					{#each items.filter((item) => item.slot === slot && item.id !== '' && item.count !== 0) as foundItem}
						<!-- <span class="absolute top-1 left-2">{foundItem.slot}</span> -->
						<img
							class="w-full h-full"
							src={`https://yamapi.chunkvault.com/images/items/${foundItem.id}`}
							alt={foundItem.id.replace('minecraft:', '')}
						/>
						<span class="absolute bottom-1 right-2 text-white drop-shadow font-bold"
							>{foundItem.count}</span
						>
					{/each}
				{/if}
			</div>
		{/each}
	</div>

	<div class="border border-gray-300 p-2 w-16 h-16 relative">
		{#if items.find((item) => item.slot === -106 && item.id !== '' && item.count !== 0)}
			{#each items.filter((item) => item.slot === -106 && item.id !== '' && item.count !== 0) as foundItem}
				<!-- <span class="absolute top-1 left-2">{foundItem.slot}</span> -->
				<img
					class="w-full h-full"
					src={`https://yamapi.chunkvault.com/images/items/${foundItem.id}`}
					alt={foundItem.id.replace('minecraft:', '')}
				/>
				<span class="absolute bottom-1 right-2 text-white drop-shadow font-bold"
					>{foundItem.count}</span
				>
			{/each}
		{/if}
	</div>
{:else}
	<p>No items found.</p>
{/if}
