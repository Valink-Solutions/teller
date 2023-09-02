<script lang="ts">
	import Icon from '@iconify/svelte';
	import { formatBytes, type WorldItem } from './utils';

	export let world: WorldItem;
</script>

<li class="card flex flex-row w-full bg-base-100 shadow-xl max-h-fit">
	<figure
		class="rounded-t-none rounded-l-sm bg-black border-r-4 min-w-fit min-h-fit h-24 w-24 max-w-[6rem] max-h-[6rem] border-black"
	>
		<img
			class="rounded-l-sm rounded-tl-sm rounded-t-none h-24 w-24 max-w-[6rem] max-h-[6rem] bg-black"
			src={world.image.length > 0
				? world.image
				: 'https://static.planetminecraft.com/files/image/minecraft/project/2020/194/13404399_l.jpg'}
			alt={world.name}
		/>
	</figure>
	<div class="flex flex-row gap-3 p-3 w-full justify-between items-center">
		<div class="flex flex-row items-center justify-between">
			<h2 class="font-bold md:w-72">
				{world.name.length > 22 ? world.name.slice(0, 22) + '...' : world.name}
			</h2>
			<span class="badge badge-primary badge-xs whitespace-nowrap w-24"
				>{formatBytes(world.size)}</span
			>
		</div>
		<!-- <h2 class="card-title">world</h2>
      <p>no backups</p> -->
		<div class="card-actions justify-around items-center">
			<div class="dropdown dropdown-end">
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label
					tabindex="0"
					class="drop-shadow-none btn btn-square btn-ghost btn-xs border-none m-1 text-lg"
				>
					<Icon icon="mdi:dots-vertical" />
				</label>
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<ul tabindex="0" class="dropdown-content z-[1] menu p-2 bg-base-100 rounded-box w-52">
					<li>
						<a href={`/local/worlds/${world.id}`} class="flex flex-row items-center">
							<Icon icon="mdi:pencil-outline" class="mr-1" />
							Quick Edit
						</a>
					</li>
					<div class="divider my-1" />
					<li>
						<button class="text-red-500 flex flex-row items-center" disabled>
							<Icon icon="mdi:trash-can-outline" class="mr-1" />
							Delete
						</button>
					</li>
				</ul>
			</div>
			<div class="join">
				<button class="btn btn-sm join-item btn-disabled" disabled>Share</button>
				<button class="btn btn-sm join-item btn-disabled" disabled>Backup</button>
				<a href={`/local/worlds/${world.id}`} class="btn btn-sm join-item">View</a>
			</div>
		</div>
	</div>
</li>
