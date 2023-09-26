<script lang="ts">
	import { openModal, closeModal, modals } from 'svelte-modals';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';
	import LocalVaultList from '../local_vault_list.svelte';
	import { localVaults, backupSettings as bSStore } from '../stores';
	import { dialog } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import type { BackupSettings } from '$lib/utils';

	export let isOpen: boolean;

	let stackIndex: number = $modals.length;

	bSStore.subscribe((val) => {
		if (val) {
			localVaults.set(
				Object.fromEntries(
					Object.entries(val.vaults).map(([name, path]) => [
						name,
						{ path: path, default: val.default_vaults?.includes(name) || false }
					])
				)
			);
		}
	});

	// localVaults.subscribe((val) => {
	// 	if (val) {
	// 		bSStore.update((bS) => {
	// 			bS.vaults = Object.fromEntries(
	// 				Object.entries(val).map(([name, vault]) => [name, vault.path])
	// 			);
	// 			bS.default_vaults = Object.entries(val)
	// 				.filter(([_, vault]) => vault.default)
	// 				.map(([name, _]) => name);
	// 			return bS;
	// 		});
	// 	}
	// });

	const addDirectory = async () => {
		let result = await dialog.open({ directory: true });
		let path = Array.isArray(result) ? result[0] : result;

		if (path) {
			localVaults.update((vaults) => {
				if (typeof path === 'string') {
					vaults[`Local Vault ${Object.keys(vaults).length + 1}`] = { path: path, default: false };
				}
				return vaults;
			});
		}
	};

	const saveDirectories = async () => {
		let res = await invoke('plugin:config|get_backup_settings');
		if (typeof res !== 'string') {
			if (res) {
				const oldBackupSettings = res as BackupSettings;
				oldBackupSettings.vaults = Object.fromEntries(
					Object.entries($localVaults).map(([name, vault]) => [name, vault.path])
				);
				oldBackupSettings.default_vaults = Object.entries($localVaults)
					.filter(([_, vault]) => vault.default)
					.map(([name, _]) => name);
				let newRes = await invoke('plugin:config|update_backup_settings', {
					settingsData: oldBackupSettings
				});
				if (typeof newRes !== 'string') {
					if (newRes) {
						toast.push('Successfully saved local vault settings.');
						const backupSettings = newRes as BackupSettings;
						localVaults.set(
							Object.fromEntries(
								Object.entries(backupSettings.vaults).map(([name, path]) => [
									name,
									{ path: path, default: backupSettings.default_vaults?.includes(name) || false }
								])
							)
						);
						closeModal();
					}
				}
			}
		}
	};
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50">
		<div
			class="card bg-slate-100 h-full w-full min-w-[25rem] max-w-[66.666667%] max-h-[85%] overflow-auto"
		>
			<div class="card-body">
				<h2 class="card-title justify-center">Manage Local Vaults</h2>

				<button on:click={addDirectory} class="btn btn-secondary">Add Directory</button>

				<LocalVaultList />

				<div class="justify-end card-actions">
					{#if stackIndex > 1}
						<button on:click={closeModal} class="btn">Close</button>
					{:else}
						<button on:click={closeModal} class="btn">Close</button>
					{/if}
					<button on:click={saveDirectories} class="btn btn-primary">Save</button>
				</div>
			</div>
		</div>
	</div>
{/if}
