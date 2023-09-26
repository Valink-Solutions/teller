<script lang="ts">
	import { openModal, closeModal, modals } from 'svelte-modals';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';
	import DirectoriesModal from '$lib/modals/directories_modal.svelte';
	import LocalVaultModal from './local_vault_modal.svelte';
	import { backupSettings } from '../stores';
	import CronSelector from '../cron_selector.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import type { BackupSettings } from '$lib/utils';

	export let isOpen: boolean;

	let stackIndex: number = $modals.length;

	onMount(async () => {
		let res = await invoke('plugin:config|get_backup_settings');
		if (typeof res !== 'string') {
			if (res) {
				backupSettings.set(res as BackupSettings);
			}
		}
	});

	function saveBackupSettings() {
		try {
			invoke('plugin:config|update_backup_settings', { settingsData: $backupSettings });
			toast.push('Successfully saved backup settings.');
		} catch (e) {
			console.error(e);
		}
	}
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50">
		<div
			class="card bg-slate-100 h-full w-full min-w-[25rem] max-w-[66.666667%] max-h-[85%] overflow-auto"
		>
			<div class="card-body gap-4">
				<h2 class="card-title justify-center">Main Settings</h2>
				<div class="flex flex-col gap-2">
					<h3 class="text-lg font-semibold">Instances</h3>
					<p class="text-sm text-gray-500">Edit where the Minecraft saves are located.</p>
					<button on:click={() => openModal(DirectoriesModal)} class="btn">Edit Directories</button>
				</div>

				<div class="divider" />

				<h3 class="card-title justify-center">Backup Settings</h3>
				<p class="text-sm text-gray-500 mx-auto">Auto Backup isn't enabled yet.</p>
				<div>
					<div class="flex flex-col gap-2">
						<span>Backup Schedule</span>
						<CronSelector />
						<div class="form-control">
							<label for="auto_backup" class="label cursor-pointer">
								<span class="label-text">Auto Backup</span>
								<input
									id="auto_backup"
									type="checkbox"
									bind:checked={$backupSettings.auto_backup}
									class="checkbox"
								/>
							</label>
						</div>
						<button on:click={saveBackupSettings} class="btn btn-primary"
							>Save Backup Settings</button
						>
					</div>
				</div>

				<div class="flex flex-row gap-4">
					<div class="flex flex-col gap-2 w-full">
						<div class="flex flex-row gap-4">
							<Icon icon="mdi:folder" class="h-12 w-12" />
							<div class="flex flex-col">
								<h3 class="text-lg font-semibold">Local Vaults</h3>
								<p class="text-sm text-gray-500">Edit the local vaults.</p>
							</div>
						</div>
						<button on:click={() => openModal(LocalVaultModal)} class="btn"
							>Edit Local Vaults</button
						>
					</div>
					<div class="flex flex-col gap-2 w-full">
						<div class="flex flex-row gap-4">
							<Icon icon="mdi:cloud" class="h-12 w-12" />
							<div class="flex flex-col">
								<h3 class="text-lg font-semibold">Remote Vaults</h3>
								<p class="text-sm text-gray-500">Edit the remote vaults.</p>
							</div>
						</div>
						<button disabled class="btn">Edit Remote Vaults</button>
					</div>
				</div>

				<div class="justify-end card-actions">
					{#if stackIndex > 1}
						<button on:click={closeModal} class="btn">Close</button>
					{:else}
						<button on:click={closeModal} class="btn">Close</button>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
