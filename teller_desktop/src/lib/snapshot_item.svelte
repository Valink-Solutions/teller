<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { SnapshotInfo } from './types/backups';
	import { formatBytes } from './utils';
	import dayjs from 'dayjs';
	import { closeModal, openModal } from 'svelte-modals';
	import DeleteModal from './modals/delete_modal.svelte';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';
	import { emit } from '@tauri-apps/api/event';
	import RestoreModal from './modals/restore_modal.svelte';

	export let snapshot: SnapshotInfo;

	export let vaultName: string;
	export let worldId: string;

	function openDeleteWindow() {
		openModal(DeleteModal, {
			deleteTitle: 'Delete Snapshot',
			deleteMessage: 'Are you sure you want to delete this snapshot?',
			deleteFunction: () => {
				invoke('plugin:backup_handler|delete_backup_from_id', {
					selectedVault: vaultName,
					worldId: worldId,
					backupId: snapshot.created.toString()
				})
					.then((res) => {
						toast.push('Successfully deleted snapshot.');
						emit('world_backup_list_updated', { worldId: worldId });
						closeModal();
					})
					.catch((err) => {
						toast.push(`Failed to delete snapshot. ${err}`, {
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

	function openRestoreModal() {
		openModal(RestoreModal, {
			worldId: worldId,
			snapshotId: snapshot.created.toString(),
			vault: vaultName
		});
	}
</script>

<li class="card flex flex-row w-full bg-base-100 shadow-xl max-h-fit">
	<div class="card-body flex-row justify-between items-center">
		<div class="grid grid-cols-3 w-full h-fit items-center gap-2">
			<div class="col-span-2 flex flex-row items-center gap-2 w-80 text-sm">
				<Icon icon="mdi:clock" />
				{dayjs(snapshot.created * 1000).format('MMMM D, YYYY [at] h:mm A')}
			</div>

			<div
				class="badge badge-primary text-black dark:text-white badge-xs font-semibold whitespace-nowrap w-24 gap-1"
			>
				<!-- <Icon icon="mdi:file" class="h-12 w-12"/> 360 -->
				{formatBytes(snapshot.size)}
			</div>
		</div>
		<div class="flex flex-row justify-around items-center gap-2">
			<div class="dropdown dropdown-end">
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label tabindex="0" class="btn btn-xs text-lg btn-ghost">
					<Icon icon="bx:dots-vertical" />
				</label>
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<ul
					tabindex="0"
					class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 mt-2 gap-2"
				>
					<li>
						<button class="flex flex-row gap-2" on:click={openRestoreModal}>
							<Icon icon="mdi:file-restore-outline" />
							Restore
						</button>
					</li>
					<div class="divider" style="margin: 2px 2px;" />
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
				<a
					href={`/local/vaults/${vaultName}/${worldId}/${snapshot.created}`}
					class="btn btn-sm join-item"
				>
					<span>View</span>
				</a>
			</div>
		</div>
		<!-- <div class="card-actions">
			<div class="join">
				<button class="btn btn-error btn-sm join-item" on:click={openDeleteWindow}>Delete</button>
				<a
					href={`/local/vaults/${vaultName}/${worldId}/${snapshot.created}`}
					class="btn btn-sm join-item">View</a
				>
			</div>
		</div> -->
	</div>
</li>
