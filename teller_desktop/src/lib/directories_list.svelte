<script lang="ts">
	import { dialog, invoke } from '@tauri-apps/api';
	import { appWindow } from '@tauri-apps/api/window';
	import { emit } from '@tauri-apps/api/event';
	import { directories, localDirs } from '$lib/stores';

	// If you can find a better way to do this please implement it
	// I'm begging you

	directories.subscribe((value) => {
		localDirs.set({ ...value });
	});

	let directoryCount = 0;

	const addDirectory = async () => {
		let path = await dialog.open({ directory: true });
		if (typeof path === 'string') {
			await invoke('check_path_for_save_folders', { path: path })
				.then((res) => {
					if (res instanceof Array) {
						// for each save location, add it to the list
						res.forEach((save_location) => {
							if (!Object.values($directories).includes(save_location)) {
								directoryCount++;
								let name: string = `Local Vault ${directoryCount}`;
								directories.update((dirs) => {
									return { ...dirs, [name]: save_location };
								});
							}
						});
					}
				})
				.catch((err) => {
					console.error(err);
				});
		} else if (typeof path === 'object' && path instanceof Array) {
			// handle multiple directories
			path.forEach(async (dir) => {
				await invoke('check_path_for_save_folders', { path: path })
					.then((res) => {
						if (res instanceof Array) {
							// for each save location, add it to the list
							res.forEach((save_location) => {
								if (!Object.values($directories).includes(save_location)) {
									directoryCount++;
									let name: string = `Local Vault ${directoryCount}`;
									directories.update((dirs) => {
										return { ...dirs, [name]: save_location };
									});
								}
							});
						}
					})
					.catch((err) => {
						console.error(err);
					});
			});
		}
	};

	const removeEntry = (name: string) => {
		directories.update((dirs) => {
			const { [name]: _, ...rest } = dirs;
			return rest;
		});
	};

	const handleNameChange = (path: string, newName: string) => {
		directories.update((dirs) => {
			const oldName = Object.keys(dirs).find((key) => dirs[key] === path);
			if (oldName) {
				const { [oldName]: _, ...rest } = dirs;
				return { ...rest, [newName]: path };
			}
			return dirs;
		});
	};

	const writeDirectories = async () => {
		let settings_data = { paths: $localDirs };
		console.log(settings_data);
		try {
			await invoke('create_saves_config', { settingsData: settings_data });

			await emit('saves_config_updated');

			await appWindow.close();
		} catch (error) {
			console.error(error);
		}
	};

	let inputValues: { [key: string]: string } = {};

	const handleBlur = (path: string, newName: string) => {
		if (newName !== inputValues[path]) {
			handleNameChange(path, newName);
			inputValues[path] = newName;
		}
	};
</script>

<div class="card min-h-full">
	<div class="card-body">
		<p class="text-center">
			Teller will attempt to walk through the selected directories and find all Minecraft save
			folders. If you have multiple save folders in a single directory, you can add them all at once
			by selecting the parent.
		</p>

		<button on:click={addDirectory} class="btn btn-secondary">Add Directory</button>

		<div class="flex flex-col gap-2">
			{#each Object.entries($localDirs) as [name, path]}
				<div class="flex flex-row gap-2 justify-between items-center w-full h-fit">
					<input
						bind:value={name}
						on:blur={(e) => {
							if (typeof path === 'string' && e.target instanceof HTMLInputElement) {
								handleBlur(path, e.target.value);
							}
						}}
						class="input"
					/>
					<div class="overflow-x-scroll overflow-y-hidden">
						<span class="w-full whitespace-nowrap">{path}</span>
					</div>
					<button on:click={() => removeEntry(name)} class="btn btn-error btn-sm">Remove</button>
				</div>
			{/each}
		</div>

		<button on:click={writeDirectories} class="btn btn-primary">Save</button>

		<h3 class="mt-4 mx-auto text-sm opacity-75 text-center">
			The way this is proccessed will probably change dramatically in future versions.
		</h3>
	</div>
</div>
