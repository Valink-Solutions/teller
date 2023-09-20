<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { currentDir, worldSortOption, worldListCache } from '$lib/stores';
	import WorldList from '$lib/world_list.svelte';
	import Icon from '@iconify/svelte';
	import type { WorldItem } from '$lib/utils';
	import { toast } from '@zerodevx/svelte-toast';

	let world_path: string = '';
	let worlds: WorldItem[] = [];

	let loading = true;
	let error = false;

	let timer: NodeJS.Timeout;

	async function handleCurrentDirChange(value: { category: string | null; path: string }) {
		loading = true;
		clearTimeout(timer);
		timer = setTimeout(async () => {
			loading = true;
			try {
				if (
					value.category === $worldListCache.category &&
					value.path === $worldListCache.instance
				) {
					worlds = $worldListCache.data;
					world_path = $worldListCache.path;
					loading = false;
					error = false;
				} else {
					const result = await invoke('plugin:config|get_folder_path', {
						dirName: value.path,
						category: value.category
					});
					if (result) {
						world_path = result as string;
						let worldResult = await invoke('plugin:folder_handler|grab_local_worlds_list', {
							localSavesPath: world_path
						});
						if (Array.isArray(worldResult)) {
							worlds = sortWorlds(worldResult, $worldSortOption);
							worldListCache.set({
								category: value.category as string,
								instance: value.path,
								path: world_path,
								data: worlds
							});
							error = false;
						} else {
							error = true;
							console.log(worldResult);
						}
					} else {
						console.log(result);
					}
				}
			} catch (err) {
				console.log(err);
				toast.push(`Error loading data: ${err}`, {
					theme: {
						'--toastBackground': '#f44336',
						'--toastProgressBackground': '#d32f2f'
					}
				});
				error = true;
			} finally {
				loading = false;
			}
		}, 750);
	}

	$: {
		currentDir.subscribe(handleCurrentDirChange);
	}

	$: {
		worldSortOption.subscribe((value) => {
			worlds = sortWorlds(worlds, value);
		});
	}

	onMount(async () => {
		if (
			$currentDir.category === $worldListCache.category &&
			$currentDir.path === $worldListCache.instance
		) {
			worlds = $worldListCache.data;
			world_path = $worldListCache.path;
			loading = false;
			error = false;
		} else {
			const result = await invoke('plugin:config|get_folder_path', {
				dirName: $currentDir.path,
				category: $currentDir.category
			});
			if (result) {
				world_path = result as string;
				let worldResult = await invoke('plugin:folder_handler|grab_local_worlds_list', {
					localSavesPath: world_path
				});
				if (Array.isArray(worldResult)) {
					worlds = sortWorlds(worldResult, $worldSortOption);
					worldListCache.set({
						category: $currentDir.category as string,
						instance: $currentDir.path,
						path: world_path,
						data: worlds
					});
					error = false;
				} else {
					error = true;
					console.log(worldResult);
				}
			} else {
				console.log(result);
			}
		}
	});

	function sortWorlds(
		worlds: WorldItem[],
		sortOption: { option: string; direction: string }
	): WorldItem[] {
		let sortedWorlds: WorldItem[] = [];
		if (sortOption.option === 'size') {
			sortedWorlds = [...worlds].sort((a, b) => a.size - b.size);
		} else if (sortOption.option === 'last_played') {
			sortedWorlds = [...worlds].sort((a, b) => {
				if (a.last_played && b.last_played) {
					return new Date(a.last_played).getTime() - new Date(b.last_played).getTime();
				} else {
					return 0;
				}
			});
		}
		if (sortOption.direction === 'desc') {
			sortedWorlds.reverse();
		}
		return sortedWorlds;
	}

	function toggleSortDirection() {
		$worldSortOption.direction = $worldSortOption.direction === 'asc' ? 'desc' : 'asc';
	}

	// This had to be done to fix path issues caused by how the functions return data
	async function openInstanceFolder(path: string) {
		await invoke('plugin:folder_handler|open_path_in_explorer', {
			path: path
		});
	}
</script>

<div class="flex flex-col h-full w-full">
	<div class="flex flex-row w-full justify-between items-center pb-2 px-2">
		<div class="flex flex-row w-full h-full items-center gap-2">
			<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
			<h1
				class="border-l-4 pl-2 w-fit max-w-[190px] xl:max-w-[240px] capitalize border-primary my-2 whitespace-nowrap text-elipsis overflow-hidden overflow-ellipsis"
			>
				{$currentDir.path}
			</h1>
			<button
				on:click={() => openInstanceFolder(world_path)}
				class="transition-opacity group flex flex-row items-center gap-1 text-xs underline whitespace-nowrap"
			>
				<span class="opacity-70">{world_path.slice(0, 20) + '...' + world_path.slice(-20)}</span>
				<Icon icon="mdi:folder-open-outline" class="opacity-0 group-hover:opacity-70" />
			</button>
		</div>
		<div class="join join-vertical lg:join-horizontal h-full items-center">
			<button on:click={toggleSortDirection} class="btn btn-secondary btn-sm">
				<Icon
					icon={$worldSortOption.direction === 'asc'
						? 'mdi:arrow-up-thick'
						: 'mdi:arrow-down-thick'}
				/>
			</button>
			<select class="select select-sm max-w-[85px] text-xs" bind:value={$worldSortOption.option}>
				<option value="size">Size</option>
				<option value="last_played">Last Played</option>
			</select>
		</div>
	</div>
	{#if loading}
		<div class="flex flex-col items-center justify-center m-auto w-full h-full">
			<Icon icon="mdi:loading" class="w-16 h-16 animate-spin" />
			<p class="text-lg font-semibold">Loading...</p>
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:error" class="w-16 h-16" />
			<p class="text-lg font-semibold">Error loading data</p>
		</div>
	{:else if world_path}
		<div class="flex px-2 h-full">
			<WorldList {worlds} on:visible currentDir={$currentDir} />
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:alert" class="w-16 h-16" />
			<p class="text-lg font-semibold">No data available</p>
		</div>
	{/if}
</div>
