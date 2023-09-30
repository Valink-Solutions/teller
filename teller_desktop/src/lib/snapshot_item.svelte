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
</script>

<li class="card flex flex-row w-full bg-base-100 shadow-xl max-h-fit">
	<div class="card-body flex-row justify-between items-center">
		<div class="flex flex-row items-center gap-2">
			<div class="flex flex-row items-center gap-2 w-80 text-sm">
				<Icon icon="mdi:clock" />
				{dayjs(snapshot.created * 1000).format('MMMM D, YYYY [at] h:mm A')}
			</div>

			<div class="badge badge-primary badge-xs font-semibold whitespace-nowrap w-24 gap-1">
				<!-- <Icon icon="mdi:file" class="h-12 w-12"/> 360 -->
				{formatBytes(snapshot.size)}
			</div>
		</div>
		<div class="card-actions">
			<div class="join">
				<button class="btn btn-error btn-sm join-item" on:click={openDeleteWindow}>Delete</button>
				<a
					href={`/local/vaults/${vaultName}/${worldId}/${snapshot.created}`}
					class="btn btn-sm join-item">View</a
				>
			</div>
		</div>
	</div>
</li>
