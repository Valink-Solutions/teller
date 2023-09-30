<script lang="ts">
	import BackupList from '$lib/backup_list.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Icon from '@iconify/svelte';
	import type { WorldItem } from '$lib/types/worlds';
	import { toast } from '@zerodevx/svelte-toast';
	import { currentVault } from '$lib/stores/navigation';
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';

	let worlds: WorldItem[] = [];

	let loading = true;
	let error = false;

	function handleBackupListUpdate(value: string | null) {
		invoke('plugin:backup_handler|grab_local_backup_list', {
			vault: value
		})
			.then((worldResult) => {
				worlds = worldResult as WorldItem[];
				error = false;
			})
			.catch((err) => {
				toast.push(`${err}`, {
					theme: {
						'--toastBackground': '#f44336',
						'--toastProgressBackground': '#d32f2f'
					}
				});
				error = true;
			})
			.finally(() => {
				loading = false;
			});
	}

	let timer: NodeJS.Timeout;

	async function handleCurrentDirChange(value: string | null) {
		if (value) {
			loading = true;
			clearTimeout(timer);
			timer = setTimeout(async () => {
				loading = true;
				handleBackupListUpdate(value);
			}, 750);
		}
	}

	$: {
		currentVault.subscribe(handleCurrentDirChange);
	}

	onMount(() => {
		listen('backup_list_updated', () => {
			handleBackupListUpdate($currentVault);
		});
	});

	async function openInstanceFolder(path: string) {
		await invoke('plugin:folder_handler|open_path_in_explorer', {
			path: path
		});
	}
</script>

<div class="flex flex-col h-full w-full">
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
	{:else}
		<div class="flex flex-row w-full justify-between items-center pb-2 px-2">
			<div class="flex flex-row w-full h-full items-center gap-2">
				<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
				<h1
					class="border-l-4 pl-2 w-fit max-w-[190px] xl:max-w-[240px] capitalize border-primary my-2 whitespace-nowrap text-elipsis overflow-hidden overflow-ellipsis"
				>
					{$currentVault}
				</h1>
				<!-- <button
					on:click={() => openInstanceFolder($currentDir.path)}
					class="transition-opacity group flex flex-row items-center gap-1 text-xs underline whitespace-nowrap"
				>
					<span class="opacity-70"
						>{$currentDir.path.slice(0, 20) + '...' + $currentDir.path.slice(-20)}</span
					>
					<Icon icon="mdi:folder-open-outline" class="opacity-0 group-hover:opacity-70" />
				</button> -->
			</div>
			<!-- <div class="join join-vertical lg:join-horizontal h-full items-center">
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
			</div> -->
		</div>
		<div class="flex px-2 h-full">
			<BackupList {worlds} on:visible currentVault={$currentVault} />
		</div>
	{/if}
</div>
