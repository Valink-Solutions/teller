<script lang="ts">
	import { closeModal, modals } from 'svelte-modals';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';
	import type { BackupSettings } from '$lib/types/backups';
	import { emit } from '@tauri-apps/api/event';
	import type { DirectorySettings } from '$lib/types/config';

	export let isOpen: boolean;

	export let worldId: string;
	export let snapshotId: string;

	export let vault: string | null;

	let stackIndex: number = $modals.length;

	let locations: {
		id: string;
		name: string;
		selected: boolean;
		instances: { id: string; name: string; selected: boolean }[];
	}[] = [];

	let replaceWorld: boolean = false;

	onMount(async () => {
		let res = await invoke('plugin:config|load_saves_folders');
		if (typeof res !== 'string') {
			if (res) {
				const directorySettings = res as DirectorySettings;
				locations.push({
					id: 'default',
					name: 'Default',
					selected: false,
					instances: [
						{
							id: 'default',
							name: 'Default',
							selected: false
						}
					]
				});
				locations = locations.concat(
					Object.entries(directorySettings.categories).map(([categoryId, vaultEntries]) => {
						const instances = Object.entries(vaultEntries.paths).map(([instanceId, _]) => ({
							id: instanceId,
							name: instanceId,
							selected: false
						}));
						return {
							id: categoryId,
							name: categoryId,
							selected: false,
							instances
						};
					})
				);
			}
		}
	});

	function restoreBackup() {
		let selectedLocations = locations.flatMap((location) => {
			let selected = [];
			if (location.selected) {
				selected.push(...location.instances.map((instance) => instance.id));
			} else {
				selected.push(
					...location.instances
						.filter((instance) => instance.selected)
						.map((instance) => instance.id)
				);
			}
			return selected;
		});
		toast.push('Restoring backup...');

		closeModal();

		invoke('plugin:backup_handler|restore_snapshot_to_world', {
			snapshotId: snapshotId,
			worldId: worldId,
			selectedVault: vault,
			instances: selectedLocations,
			replace: replaceWorld
		});
	}
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50">
		<div class="card bg-base-100 min-w-[25rem] max-w-[66.666667%]">
			<div class="card-body">
				<h2 class="card-title">Restore Options</h2>
				<p class="card-text">How would you like to restore this world?</p>
				<p class="mt-2">Select restore locations:</p>

				<div class="flex flex-col max-h-[400px] overflow-y-auto overflow-x-hidden">
					<ul class="menu bg-base-200 rounded-box flex flex-col w-full">
						{#each locations as location (location.id)}
							{#if location.id !== 'default'}
								<li class="flex flex-row w-full">
									<details class="w-full">
										<summary>
											<div class="form-control">
												<input type="checkbox" class="checkbox" bind:checked={location.selected} />
											</div>
											{location.name}
										</summary>
										<ul>
											{#each location.instances as instance (instance.id)}
												<li class="flex flex-row items-center gap-2">
													<div class="w-full">
														<input
															type="checkbox"
															class="checkbox"
															bind:checked={instance.selected}
														/>
														{instance.name}
													</div>
												</li>
											{/each}
										</ul>
									</details>
								</li>
							{:else}
								<li class="flex flex-row items-center">
									<div class="w-full">
										<input type="checkbox" class="checkbox" bind:checked={location.selected} />
										{location.name}
									</div>
								</li>
							{/if}
						{/each}
					</ul>
				</div>

				<div class="justify-between card-actions">
					<div class="form-control">
						<label class="label cursor-pointer gap-4">
							<span class="label-text">Replace world</span>
							<input type="checkbox" class="toggle" bind:checked={replaceWorld} />
						</label>
					</div>
					<div class="flex flex-row gap-2">
						{#if stackIndex > 1}
							<button on:click={closeModal} class="btn">Close</button>
						{:else}
							<button on:click={closeModal} class="btn">Close</button>
						{/if}

						<button on:click={restoreBackup} class="btn btn-primary">Restore</button>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
