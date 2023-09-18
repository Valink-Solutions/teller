<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window';
	import DirectoriesList from '$lib/directories_list.svelte';
	import { dialog, invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { emit } from '@tauri-apps/api/event';
	import { directorySettings } from '$lib/stores';
	import type { DirectorySettings } from '$lib/utils';

	let directoryCount = 0;

	onMount(async () => {
		appWindow.setResizable(false).catch((err) => {
			console.error(err);
		});

		await appWindow.setTitle('Set World Save Directories').catch((err) => {
			console.error(err);
		});

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

			await appWindow.close();
		} catch (error) {
			console.error(error);
		}
	};

	const addDirectory = async () => {
		let path = await dialog.open({ directory: true });
		if (typeof path === 'string') {
			await invoke('plugin:folder_handler|check_path_for_save_folders', { path: path })
				.then((res) => {
					if (res instanceof Array) {
						// for each save location, add it to the list
						directoryCount++;
						let directoryName: string = `Local Vault ${directoryCount}`;
						let saveLocationCount = 0;
						res.forEach((save_location) => {
							saveLocationCount++;
							let name: string = `Save ${saveLocationCount}`;
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
</script>

<div class="flex flex-col gap-4 p-4 min-h-screen h-full flex-wrap">
	<button on:click={addDirectory} class="btn btn-secondary">Add Directory</button>
	<DirectoriesList />
	<button on:click={writeDirectories} class="btn btn-primary">Save</button>
</div>
