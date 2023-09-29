<script lang="ts">
	import Icon from '@iconify/svelte';
	import { formatBytes } from './utils';
	import type { CurrentDir } from './types/navigation';
	import type { WorldItem } from './types/worlds';

	export let world: WorldItem;
	export let currentVault: string | null;
</script>

<li class="card flex flex-row w-full bg-base-100 shadow-xl max-h-fit">
	<figure
		class="bg-black border-r-4 min-w-fit min-h-fit h-24 w-24 max-w-[6rem] max-h-[6rem] border-black"
	>
		<img
			class="h-24 w-24 max-w-[6rem] max-h-[6rem] bg-black"
			src={world.image.length > 0
				? world.image
				: 'https://static.planetminecraft.com/files/image/minecraft/project/2020/194/13404399_l.jpg'}
			alt={world.name}
		/>
	</figure>
	<div class="flex flex-row gap-3 p-3 w-full justify-between items-center">
		<div class="flex flex-row items-center justify-between gap-2">
			<div class="w-56 xl:w-80 2xl:w-96">
				<h2 class="font-bold w-full overflow-hidden overflow-ellipsis whitespace-nowrap">
					{world.name}
				</h2>
			</div>

			<span class="badge badge-primary badge-xs font-semibold whitespace-nowrap w-24"
				>{formatBytes(world.size)}</span
			>
		</div>
		<!-- <h2 class="card-title">world</h2>
      <p>no backups</p> -->
		<div class="card-actions justify-around items-center">
			<div class="join">
				<button class="btn btn-warning btn-sm join-item" disabled>Delete</button>
				<a href={`/local/vaults/${currentVault}/${world.id}`} class="btn btn-sm join-item">View</a>
			</div>
		</div>
	</div>
</li>
