<script lang="ts">
	import { closeModal, modals } from 'svelte-modals';
	import DirectoriesList from '$lib/directories_list.svelte';
	import { dialog, invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { emit } from '@tauri-apps/api/event';
	import { directorySettings } from '$lib/stores/settings';
	import type { DirectorySettings } from '$lib/types/config';
	import { toast } from '@zerodevx/svelte-toast';

	let directoryCount = 0;

	onMount(async () => {
		try {
			let savesFolders = await invoke('plugin:config|load_saves_folders');
			if (typeof savesFolders === 'string') {
				console.error(`Failed to load saves folders: ${savesFolders}`);
			} else {
				directorySettings.set(savesFolders as DirectorySettings);
			}
		} catch (error) {
			console.error(`Error invoking load_saves_folders: ${error}`);
		}
	});

	const writeDirectories = async () => {
		console.log($directorySettings);
		try {
			await invoke('plugin:config|update_saves_config', { settingsData: $directorySettings });

			await emit('saves_config_updated');

			closeModal();
		} catch (error) {
			console.error(error);
			toast.push(`Error saving directories: ${error}`, {
				theme: {
					'--toastBackground': '#f44336',
					'--toastProgressBackground': '#d32f2f'
				}
			});
		}
	};

	const addDirectory = async () => {
		let path = await dialog.open({ directory: true });
		if (typeof path === 'string') {
			await invoke('plugin:folder_handler|check_path_for_save_folders', { path: path })
				.then((res) => {
					if (res instanceof Array) {
						// for each save location, add it to the list
						let directoryName: string = `Instance ${
							Object.keys($directorySettings.categories).length + 1
						}`;
						res.forEach((save_location, index) => {
							let name: string = `Save ${index + 1}`;
							directorySettings.update((dirs) => {
								let newCategory = { ...dirs.categories };
								if (!newCategory[directoryName]) {
									newCategory[directoryName] = { paths: {} };
								}
								newCategory[directoryName].paths[name] = save_location;
								return { ...dirs, categories: newCategory };
							});
						});
					}
				})
				.catch((err) => {
					console.error(err);
				});
		}
	};

	export let isOpen: boolean;

	let stackIndex: number = $modals.length;
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50 py-2">
		<div
			class="card bg-slate-100 h-full w-full min-w-[25rem] max-w-[66.666667%] max-h-[85%] overflow-auto"
		>
			<div class="card-body">
				<div class="flex flex-col gap-2">
					<h2 class="card-title">Manage Local Instances</h2>
					<p class="text-sm">
						Our system will attempt to find as many Minecraft instances within the directory
						provided.
					</p>
				</div>
				<button on:click={addDirectory} class="btn btn-secondary">Add Instance</button>

				<div class="flex w-full">
					<DirectoriesList />
				</div>

				<div class="justify-end card-actions">
					{#if stackIndex > 1}
						<button on:click={closeModal} class="btn">Close</button>
					{:else}
						<button on:click={closeModal} class="btn">Close</button>
					{/if}
					<button on:click={writeDirectories} class="btn btn-primary">Save</button>
				</div>
			</div>
		</div>
	</div>
{/if}
