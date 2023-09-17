<script lang="ts">
	import { dialog, invoke } from '@tauri-apps/api';
	import { appWindow } from '@tauri-apps/api/window';
	import { emit } from '@tauri-apps/api/event';
	import Sortable from 'sortablejs';
	import { onMount, afterUpdate } from 'svelte';
	import { writable } from 'svelte/store';
	import type { DirectorySettings, VaultEntries } from './utils.ts';
	import Icon from '@iconify/svelte';

	// If you can find a better way to do this please implement it
	// I'm begging you

	let categories: Record<string, VaultEntries>;
	let directoryCount = 0;

	const directorySettings = writable<DirectorySettings>({ categories: {} });

	directorySettings.subscribe((value) => {
		categories = value.categories;
	});

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

	const deleteDirectory = (category: string, path?: string) => {
		directorySettings.update((dirs) => {
			let newCategory = { ...dirs.categories };
			if (newCategory[category]) {
				if (path) {
					// delete specific path from category
					newCategory[category].paths = Object.fromEntries(
						Object.entries(newCategory[category].paths).filter(([key, value]) => value !== path)
					);
				} else {
					// delete entire category
					delete newCategory[category];
				}
			}
			return { ...dirs, categories: newCategory };
		});
	};

	onMount(async function () {
		Sortable.create(document.getElementById('categories'), {
			group: {
				name: 'categories',
				put: false,
				pull: false
			},
			animation: 200
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

	afterUpdate(() => {
		Object.entries(categories).forEach(([category, _]) => {
			const checkboxElement = document.getElementById(`checkbox-${category}`);
			const categoryElement = document.getElementById(category);

			if (categoryElement && checkboxElement && categoryElement.parentNode) {
				const parentDiv = categoryElement.parentNode.parentNode;

				Sortable.create(checkboxElement, {
					group: {
						name: 'paths',
						put: true,
						pull: true
					},
					onAdd: function (evt: any) {
						// Get the old category from the evt.from.id
						const oldCategory = evt.from.id;
						// Get the path name from the evt.item.dataset.path
						let pathName = evt.item.dataset.path;
						// Get the path data from the evt.item.dataset.pathData
						let pathData = evt.item.dataset.pathData;
						let newPathName = pathName;
						// Update the categories object
						directorySettings.update((dirs) => {
							let newCategory = { ...dirs.categories };
							let copyCount = 1;
							// Check if the new item has the same name as an item already in the list
							while (newCategory[category].paths.hasOwnProperty(newPathName)) {
								// Append " (copy x)" to the name
								newPathName = `${pathName} (copy ${copyCount})`;
								copyCount++;
							}
							// Add the path to the new category
							newCategory[category].paths[newPathName] = pathData;
							// Remove the path from the old category
							delete newCategory[oldCategory].paths[evt.item.dataset.path];
							return { ...dirs, categories: newCategory };
						});
						// Update the path in the DOM
						evt.item.dataset.path = newPathName;
						evt.item.dataset.pathData = pathData;
						// Move the added item to the correct location in the DOM
						categoryElement.appendChild(evt.item);
					}
				});

				Sortable.create(categoryElement, {
					group: {
						name: 'paths',
						put: true,
						pull: true
					},
					onRemove: function (evt: any) {
						if (!categoryElement.children.length) {
						}
					}
				});
			}
		});
	});

	const updateCategoryName = (oldName: string, newName: string) => {
		if (oldName !== newName) {
			directorySettings.update((dirs) => {
				let newCategory = { ...dirs.categories };
				newCategory[newName] = newCategory[oldName];
				delete newCategory[oldName];
				return { ...dirs, categories: newCategory };
			});
		}
	};

	const updatePathName = (category: string, oldName: string, newName: string) => {
		if (oldName !== newName) {
			directorySettings.update((dirs) => {
				let newCategory = { ...dirs.categories };
				newCategory[category].paths[newName] = newCategory[category].paths[oldName];
				delete newCategory[category].paths[oldName];
				return { ...dirs, categories: newCategory };
			});
		}
	};

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
</script>

<div class="flex flex-col gap-4 p-4 min-h-screen h-full flex-wrap">
	<button on:click={addDirectory} class="btn btn-secondary">Add Directory</button>

	<div class="flex flex-grow flex-col gap-4 min-h-full h-full overflow-auto" id="categories">
		{#each Object.entries($directorySettings.categories) as [category, value], i (category)}
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<div data-category={category} class="collapse collapse-arrow max-w-full">
				<input id={`checkbox-${category}`} type="checkbox" />
				<div class="collapse-title text-xl font-medium">
					<button
						style="position: absolute; z-index: 1;"
						on:click={() => deleteDirectory(category)}
						class="btn btn-sm btn-error hover:bg-red-700 z-auto"
					>
						<Icon icon="mdi:trash-can-outline" />
					</button>
					<!-- svelte-ignore a11y-no-static-element-interactions -->
					<span
						class="ml-14 w-full max-w-lg overflow-x-auto whitespace-nowrap"
						style="position: absolute; z-index: 1;"
						contenteditable="true"
						on:keydown={(e) => {
							if (e.key === 'Enter') {
								e.preventDefault();
								e.target.blur();
							}
						}}
						on:blur={(e) => updateCategoryName(category, e.target.textContent)}
					>
						{category}
					</span>
				</div>
				<div id={category} class="collapse-content flex flex-col max-w-full gap-2">
					{#each Object.entries(value.paths) as [pathName, pathValue] (pathName)}
						<div
							data-path={pathName}
							data-path-data={pathValue}
							class="flex flex-row gap-2 card items-center max-w-full h-fit odd:bg-slate-200 p-2"
						>
							<button
								on:click={() => deleteDirectory(category, pathValue)}
								class="btn btn-sm btn-ghost"
							>
								<Icon icon="mdi:close-thick" />
							</button>
							<!-- svelte-ignore a11y-no-static-element-interactions -->
							<span
								class="whitespace-nowrap w-24 overflow-x-auto"
								contenteditable="true"
								on:keydown={(e) => {
									if (e.key === 'Enter') {
										e.preventDefault();
										e.target.blur();
									}
								}}
								on:blur={(e) => updatePathName(category, pathName, e.target.textContent)}
							>
								{pathName}
							</span>
							<!-- <span class="overflow-x-scroll">{pathValue}</span> -->
							<div
								class="overflow-x-auto overflow-y-hidden h-fit max-w-md md:max-w-lg lg:max-w-xl d-flex align-items-center justify-content-center"
							>
								<span class="w-full whitespace-nowrap px-2">{pathValue}</span>
							</div>
							<Icon icon="mdi:drag" class="cursor-move ml-auto opacity-50" />
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
	<button on:click={writeDirectories} class="btn btn-primary">Save</button>
</div>
