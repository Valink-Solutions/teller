<script lang="ts">
	import { closeModal, modals } from 'svelte-modals';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';
	import type { BackupSettings } from '$lib/types/backups';
	import { emit } from '@tauri-apps/api/event';

	export let isOpen: boolean;

	export let worldName: string = 'World';
	export let worldId: string;

	export let category: string | null;
	export let instance: string | null;

	let stackIndex: number = $modals.length;

	let locations: { id: string; name: string; selected: boolean }[] = [];

	onMount(async () => {
		let res = await invoke('plugin:config|get_backup_settings');
		if (typeof res !== 'string') {
			if (res) {
				const backupSettings = res as BackupSettings;
				locations = Object.entries(backupSettings.vaults).map(([id, _]) => ({
					id,
					name: id,
					selected: backupSettings.default_vaults?.includes(id) || false
				}));
			}
		}
	});

	function backupWorld() {
		let selectedLocations = locations
			.filter((location) => location.selected)
			.map((location) => location.id);
		toast.push('Creating backup...');

		closeModal();

		invoke('plugin:backup_handler|create_backup_from_id', {
			worldId: worldId,
			category: category,
			instance: instance,
			vaults: selectedLocations
		});
	}
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50">
		<div class="card bg-base-100 min-w-[25rem] max-w-[66.666667%]">
			<div class="card-body">
				<h2 class="card-title">Backup Options</h2>
				<p class="card-text">
					You are about to create a backup of <strong>{worldName}</strong>.
				</p>
				<p class="mt-4">Select backup locations:</p>
				<ul class="bg-base-200 p-2">
					{#each locations as location (location.id)}
						<li>
							<div class="form-control">
								<label class="label cursor-pointer">
									<span class="label-text">{location.name}</span>
									<input type="checkbox" class="checkbox" bind:checked={location.selected} />
								</label>
							</div>
							<!-- <label>
								<input type="checkbox" bind:checked={location.selected} on:change={() => handleLocationChange(location.id)} />
								{location.name}
							</label> -->
						</li>
					{/each}
				</ul>

				<div class="justify-end card-actions">
					{#if stackIndex > 1}
						<button on:click={closeModal} class="btn">Close</button>
					{:else}
						<button on:click={closeModal} class="btn">Close</button>
					{/if}
					<button on:click={backupWorld} class="btn btn-primary">Backup</button>
				</div>
			</div>
		</div>
	</div>
{/if}
