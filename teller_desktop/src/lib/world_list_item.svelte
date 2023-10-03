<script lang="ts">
	import Icon from '@iconify/svelte';
	import { formatBytes } from './utils';
	import type { CurrentDir } from './types/navigation';

	import { closeModal, openModal } from 'svelte-modals';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';
	import DeleteModal from './modals/delete_modal.svelte';
	import BackupModal from './modals/backup_modal.svelte';
	import type { WorldItem } from './types/worlds';
	import { emit } from '@tauri-apps/api/event';

	export let world: WorldItem;
	export let currentDir: CurrentDir = { path: 'default', category: null };

	function openBackupWindow() {
		openModal(BackupModal, {
			worldName: world.name,
			worldId: world.id,
			category: currentDir.category,
			instance: currentDir.path
		});
	}

	function openDeleteWindow() {
		openModal(DeleteModal, {
			deleteTitle: 'Delete World',
			deleteMessage: 'Are you sure you want to delete this world?',
			deleteFunction: () => {
				invoke('plugin:world_handler|delete_world_by_id', {
					worldId: world.id,
					category: currentDir.category,
					instance: currentDir.path
				})
					.then((res) => {
						toast.push(`Successfully deleted ${world.name}`);
						emit('world_list_updated');
						closeModal();
					})
					.catch((err) => {
						toast.push(`Failed to delete ${world.name}. ${err}`, {
							theme: {
								'--toastBackground': '#EF4444',
								'--toastProgressBackground': '#F87171',
								'--toastProgressText': '#fff',
								'--toastText': '#fff'
							}
						});
					});
			}
		});
	}
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
			<div class="dropdown dropdown-end">
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label tabindex="0" class="btn btn-xs text-lg btn-ghost">
					<Icon icon="bx:dots-vertical" />
				</label>
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<ul
					tabindex="0"
					class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 mt-2"
				>
					<li>
						<button
							class="flex flex-row gap-2 text-red-500 hover:text-red-700"
							on:click={openDeleteWindow}
						>
							<Icon icon="mdi:trash-can-outline" />
							<span>Delete</span>
						</button>
					</li>
				</ul>
			</div>
			<div class="join">
				<!-- <button class="btn btn-error btn-sm join-item" on:click={openDeleteWindow}>Delete</button> -->
				<button on:click={openBackupWindow} class="btn btn-sm join-item">Backup</button>
				<a
					href={`/local/worlds/${currentDir.category}/${currentDir.path}/${world.id}`}
					class="btn btn-sm join-item">View</a
				>
			</div>
		</div>
	</div>
</li>
