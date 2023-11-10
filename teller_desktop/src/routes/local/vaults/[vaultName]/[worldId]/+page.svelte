<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import dayjs from 'dayjs';
	import type { BackupMetadata, SnapshotInfo } from '$lib/types/backups';
	import SnapshotItem from '$lib/snapshot_item.svelte';

	let world_data: BackupMetadata;

	let snapshots: SnapshotInfo[] = [];

	let loading = true;
	let error = false;

	function handleSnapshotsUpdate() {
		invoke('plugin:backup_handler|grab_world_backups', {
			worldId: $page.params.worldId,
			selectedVault: $page.params.vaultName
		})
			.then((result) => {
				snapshots = result as SnapshotInfo[];
				loading = false;
			})
			.catch((err) => {
				console.log(err);
				error = true;
			});
	}

	interface backupListUpdate {
		worldId: string;
	}

	onMount(() => {
		invoke('plugin:backup_handler|grab_world_metadata', {
			worldId: $page.params.worldId,
			selectedVault: $page.params.vaultName
		})
			.then((result) => {
				world_data = result as BackupMetadata;
				handleSnapshotsUpdate();
			})
			.catch((err) => {
				console.log(err);
				error = true;
			});

		listen<backupListUpdate>('world_backup_list_updated', (event) => {
			if (event.payload.worldId === $page.params.worldId) {
				handleSnapshotsUpdate();
			}
		});
	});
</script>

<div class="flex flex-col justify-start w-full px-4 gap-4">
	<div class="flex flex-row justify-between items-center">
		<button
			class="btn btn-ghost w-20"
			on:click={() => goto(`/local/vaults/${$page.params.categoryName}`)}
		>
			<Icon icon="mdi:arrow-left" class="w-6 h-6" />
		</button>
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
	{:else if world_data}
		<div class="flex flex-row space-x-4">
			<div class="relative w-28 h-24">
				<img
					src={world_data.data.icon
						? world_data.data.icon
						: 'https://static.planetminecraft.com/files/image/minecraft/project/2020/194/13404399_l.jpg'}
					alt={world_data.data.name}
					class="object-cover w-full h-full self-start border-4 border-black shadow-neu"
				/>
				<div class="badge badge-xs badge-ghost absolute -bottom-2 left-0 right-0 mx-auto">
					{world_data.data.game_engine}
				</div>
			</div>
			<div class="flex flex-col w-full">
				<h1 class="text-4xl font-bold mb-2">{world_data.data.name}</h1>
				<p class="flex flex-row items-center text-sm mb-1 gap-2">
					<Icon icon="mdi:calendar-clock" class="mr-1" />
					<span class="font-semibold">Last Played:</span>
					{dayjs(world_data.data.last_played).format('MMMM D, YYYY [at] h:mm A')}
				</p>
				<div class="flex flex-row items-center justify-between">
					<p class="flex flex-row items-center mb-1 gap-2">
						<Icon icon="mdi:gamepad-variant" class="mr-1" />
						<span class="font-semibold">Game Type:</span>
						{world_data.data.game_type}
					</p>
					<p class="flex flex-row items-center mb-1 gap-2">
						<Icon icon="mdi:shield-outline" class="mr-1" />
						<span class="font-semibold">Difficulty:</span>
						{world_data.data.difficulty}
					</p>
				</div>
			</div>
		</div>

		{#if snapshots}
			<div class="flex flex-row justify-between items-center">
				<h1 class="border-l-4 pl-2 border-primary text-lg font-bold">All Backups</h1>
				<button class="btn btn-sm">
					<Icon icon="material-symbols:directory-sync" on:click={handleSnapshotsUpdate} />
				</button>
			</div>
			<ul class="flex flex-col gap-4">
				{#each snapshots as snapshot}
					<SnapshotItem
						{snapshot}
						vaultName={$page.params.vaultName}
						worldId={$page.params.worldId}
					/>
				{/each}
			</ul>
		{/if}
	{:else}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:alert" class="w-16 h-16" />
			<p class="text-lg font-semibold">No data available</p>
		</div>
	{/if}
</div>
