<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import dayjs from 'dayjs';
	import type { BackupMetadata } from '$lib/types/backups';
	import { openModal } from 'svelte-modals';
	import RestoreModal from '$lib/modals/restore_modal.svelte';
	import { writable } from 'svelte/store';

	let world_metadata: BackupMetadata;

	let currentPage = 1;
	const itemsPerPage = 6;

	let loading = true;
	let error = false;

	// $: paginatedPlayers =
	// 	world_metadata && world_metadata.data.players
	// 		? world_metadata.data.players.slice(
	// 				(currentPage - 1) * itemsPerPage,
	// 				currentPage * itemsPerPage
	// 		  )
	// 		: [];

	let paginatedPlayersStore = writable<any[]>([]);

	$: {
		if (world_metadata && world_metadata.data.players) {
			const start = (currentPage - 1) * itemsPerPage;
			const end = currentPage * itemsPerPage;
			const players = world_metadata.data.players.slice(start, end);
			fetchUsernames(players);
		} else {
			paginatedPlayersStore.set([]);
		}
	}

	async function fetchUsernames(players: any[]) {
		const playersWithUsernames = await Promise.all(
			players.map(async (player) => {
				try {
					if (player.id === '~local_player') {
						player.username = 'Local Player';
						return player;
					}
					const response = await fetch(`https://playerdb.co/api/player/minecraft/${player.id}`);
					if (response.ok) {
						const data = await response.json();
						if (data.success) {
							player.username = data.data.player.username;
							// player.avatar = data.data.player.avatar;
						} else {
							player.username = 'Player';
						}
					} else {
						player.username = 'Player';
						return player;
					}
				} catch (error) {
					console.error('Error fetching player data:', error);
					player.username = 'Player';
				}
				return player;
			})
		);
		paginatedPlayersStore.set(playersWithUsernames);
	}

	onMount(async () => {
		try {
			const res = await invoke('plugin:backup_handler|grab_backup_metadata', {
				worldId: $page.params.worldId,
				selectedVault: $page.params.vaultName,
				backupId: $page.params.snapshotId
			});
			if (res) {
				world_metadata = res as BackupMetadata;
			}
			loading = false;
		} catch (err) {
			console.log(err);
			error = true;
		}
	});

	function openRestoreModal() {
		openModal(RestoreModal, {
			worldId: $page.params.worldId,
			snapshotId: $page.params.snapshotId,
			vault: $page.params.vaultName
		});
	}
</script>

<div class="flex flex-col justify-start w-full px-4 gap-4">
	<div class="flex flex-row justify-between items-center">
		<button
			class="btn btn-ghost w-20"
			on:click={() => goto(`/local/vaults/${$page.params.vaultName}/${$page.params.worldId}`)}
		>
			<Icon icon="mdi:arrow-left" class="w-6 h-6" />
		</button>

		<button class="btn btn-sm btn-secondary" on:click={openRestoreModal}> Restore Backup </button>
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
	{:else if world_metadata}
		<div class="flex flex-row items-center space-x-4">
			<div class="relative w-28 h-24">
				<img
					src={world_metadata.data.icon
						? world_metadata.data.icon
						: 'https://static.planetminecraft.com/files/image/minecraft/project/2020/194/13404399_l.jpg'}
					alt={world_metadata.data.name}
					class="object-cover w-full h-full self-start border-4 border-black shadow-neu"
				/>
				<div class="badge badge-xs absolute -bottom-2 left-0 right-0 mx-auto">
					{world_metadata.data.game_engine}
				</div>
			</div>
			<div class="flex flex-col w-full">
				<h1 class="text-4xl font-bold mb-2">{world_metadata.data.name}</h1>
				<p class="flex flex-row items-center text-sm mb-1 gap-2">
					<Icon icon="mdi:calendar-clock" class="mr-1" />
					<span class="font-semibold">Last Played:</span>
					{dayjs(world_metadata.data.last_played).format('MMMM D, YYYY [at] h:mm A')}
				</p>
				<div class="flex flex-row items-center justify-between">
					<p class="flex flex-row items-center mb-1 gap-2">
						<Icon icon="mdi:gamepad-variant" class="mr-1" />
						<span class="font-semibold">Game Type:</span>
						{world_metadata.data.game_type}
					</p>
					<p class="flex flex-row items-center mb-1 gap-2">
						<Icon icon="mdi:shield-outline" class="mr-1" />
						<span class="font-semibold">Difficulty:</span>
						{world_metadata.data.difficulty}
					</p>
				</div>
			</div>
		</div>

		{#if world_metadata.data.players.length >= 1}
			<div class="flex flex-row justify-between items-center">
				<h1 class="border-l-4 pl-2 border-primary text-lg font-bold">Players</h1>

				<div class="flex flex-row justify-center items-center space-x-4">
					<button
						on:click={() => (currentPage = Math.max(1, currentPage - 1))}
						class="btn btn-sm btn-primary"
						disabled={currentPage === 1}>Prev</button
					>
					<span class="text-sm font-bold">Page {currentPage}</span>
					<button
						on:click={() =>
							(currentPage = Math.min(
								Math.ceil(world_metadata.data.players.length / itemsPerPage),
								currentPage + 1
							))}
						class="btn btn-sm btn-primary"
						disabled={currentPage === Math.ceil(world_metadata.data.players.length / itemsPerPage)}
						>Next</button
					>
				</div>
			</div>

			<div class="grid grid-cols-2 xl:grid-cols-3 gap-4 2xl:align-start">
				{#each $paginatedPlayersStore as player}
					<div class="card p-4 flex flex-row justify-between select-none">
						<div class="flex flex-row items-center">
							<img
								src={player.avatar
									? player.avatar
									: 'https://api.mineatar.io/face/8667ba71b85a4004af54457a9734eed7?scale=32&overlay=false'}
								alt={player.username ? player.username : 'Default Icon'}
								class="w-8 h-8 mr-2"
							/>
							{player.username ? player.username : player.id}
						</div>

						<!-- <a
							class="btn btn-ghost"
							href={`/local/worlds/${$page.params.categoryName}/${$page.params.pathName}/${$page.params.worldId}/player/${player.id}`}
						>
							<Icon icon="mdi:arrow-right" />
						</a> -->
					</div>
				{/each}
			</div>
		{/if}

		{#if world_metadata.data.game_rules}
			<h1 class="border-l-4 pl-2 border-primary text-lg font-bold">Game Rules</h1>
			<div class="grid grid-cols-2 xl:grid-cols-3 gap-4">
				{#each Object.entries(world_metadata.data.game_rules) as [rule, value]}
					<div class="card flex flex-row p-2 justify-between items-center bg-slate-200">
						<span class="text-sm font-semibold">{rule}:</span>
						{#if value === true || value === false}
							<div class="flex flex-row w-12 relative">
								<div
									class="h-3 w-full bg-white border-[3px] border-black drop-shadow-neu-pressed"
								/>
								<div
									class="-top-1 h-[20px] w-[20px] absolute border-[3px] border-black drop-shadow-neu-pressed {value ===
									true
										? 'bg-green-600 right-0'
										: 'bg-red-600 left-0'}"
								/>
							</div>
						{:else}
							<span
								class="min-w-[48px] px-2 text-center border-2 border-black bg-white drop-shadow-neu-pressed"
								>{value}</span
							>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	{:else}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:alert" class="w-16 h-16" />
			<p class="text-lg font-semibold">No data available</p>
		</div>
	{/if}
</div>
