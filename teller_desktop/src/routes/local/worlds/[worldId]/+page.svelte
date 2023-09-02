<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import dayjs from 'dayjs';
	import InventoryViewer from '$lib/inventory_viewer.svelte';

	let world_data: any;
	let player_data: any;

	let currentPage = 1;
	const itemsPerPage = 4;

	let loading = true;
	let error = false;

	$: paginatedPlayers =
		world_data && world_data.players
			? world_data.players.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
			: [];

	onMount(async () => {
		try {
			const res = await invoke('get_world_by_id', { worldId: $page.params.worldId });
			world_data = res;

			if (world_data) {
				if (world_data.game_engine === 'Java') {
					const players: Record<string, any> = await invoke('grab_player_meta_from_uuids', {
						playerDataList: world_data.players
					});
					player_data = Object.keys(players).reduce((acc: Record<string, any>, uuid: string) => {
						acc[uuid] = players[uuid];
						return acc;
					}, {});
				}
			}
		} catch (err) {
			console.log(err);
			error = true;
			// goto('/local');
		} finally {
			loading = false;
		}
	});
</script>

<div class="flex flex-col justify-start w-full px-4 gap-4">
	<button class="btn btn-ghost w-20" on:click={() => goto('/local')}>
		<Icon icon="mdi:arrow-left" class="w-6 h-6" />
	</button>

	{#if loading}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:loading" class="w-16 h-16 animate-spin" />
			<p class="text-lg font-semibold">Loading...</p>
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:error" class="w-16 h-16" />
			<p class="text-lg font-semibold">Error loading data</p>
		</div>
	{:else if world_data}
		<div class="flex flex-row items-center space-x-4">
			<img
				src={world_data.icon.length > 0
					? world_data.icon
					: 'https://static.planetminecraft.com/files/image/minecraft/project/2020/194/13404399_l.jpg'}
				alt={world_data.name}
				class="object-cover w-24 h-24 self-start"
			/>
			<div class="flex flex-col w-full">
				<h1 class="text-4xl font-bold mb-2">{world_data.name}</h1>
				<p class="flex flex-row items-center text-sm text-gray-600 mb-1 gap-2">
					<Icon icon="mdi:calendar-clock" class="mr-1" />
					<span class="font-semibold">Last Played:</span>
					{dayjs(world_data.last_played).format('MMMM D, YYYY [at] h:mm A')}
				</p>
				<div class="flex flex-row items-center justify-between">
					<p class="flex flex-row items-center text-gray-600 mb-1 gap-2">
						<Icon icon="mdi:gamepad-variant" class="mr-1" />
						<span class="font-semibold">Game Type:</span>
						{world_data.game_type}
					</p>
					<p class="flex flex-row items-center text-gray-600 mb-1 gap-2">
						<Icon icon="mdi:shield-outline" class="mr-1" />
						<span class="font-semibold">Difficulty:</span>
						{world_data.difficulty}
					</p>
				</div>
			</div>
		</div>

		<div class="flex flex-row justify-between items-center">
			<h1 class="border-l-primary border-l-4 pl-3 text-lg font-bold">Players</h1>

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
							Math.ceil(world_data.players.length / itemsPerPage),
							currentPage + 1
						))}
					class="btn btn-sm btn-primary"
					disabled={currentPage === Math.ceil(world_data.players.length / itemsPerPage)}
					>Next</button
				>
			</div>
		</div>
		<div class="flex flex-col 2xl:grid 2xl:grid-cols-2 gap-4 2xl:align-start">
			{#each paginatedPlayers as player}
				<div class="collapse collapse-arrow border-4 border-black drop-shadow-neu">
					<input type="checkbox" />
					<div class="collapse-title text-xl font-medium flex items-center">
						<img
							src={player_data
								? player_data[player.id].avatar
								: 'https://api.mineatar.io/face/8667ba71b85a4004af54457a9734eed7?scale=32&overlay=false'}
							alt={player_data ? player_data[player.id].username : 'Default Icon'}
							class="w-8 h-8 mr-2"
						/>
						{player_data ? player_data[player.id].username : player.id}
					</div>
					<div class="collapse-content gap-4">
						{#if player_data}
							<div class="flex flex-col gap-2">
								<div class="flex select-none w-full justify-between">
									{#each Array(Math.floor(player.health / 2)) as _}
										<Icon icon="mdi:heart" class="w-6 h-6 text-red-500" />
									{/each}
									{#if player.health % 2}
										<Icon icon="mdi:heart-half" class="w-6 h-6 text-red-500" />
									{/if}
									{#each Array(10 - Math.ceil(player.health / 2)) as _}
										<Icon icon="mdi:heart" class="w-6 h-6 text-gray-500" />
									{/each}
								</div>
								<div class="flex flex-row items-center gap-4">
									<p class="text-xs text-opacity-50 whitespace-nowrap">Level {player.level}</p>
									<progress
										class="progress progress-primary w-full"
										value={player.xp.toFixed(2)}
										max="100"
									/>
								</div>
								<InventoryViewer items={player.inventory} />
							</div>
						{:else}
							<div class="flex flex-col">
								<div class="flex flex-row gap-4 items-center">
									<p class="whitespace-nowrap text-xs text-opacity-50">Level: {player.level}</p>
									<progress
										class="progress progress-primary w-full"
										value={player.xp.toFixed(2)}
										max="100"
									/>
								</div>
								<InventoryViewer items={player.inventory} />
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		<h1 class="border-l-primary border-l-4 pl-3 text-lg font-bold">World Data</h1>
		<div class="collapse collapse-arrow border-4 border-black drop-shadow-neu">
			<input type="checkbox" />
			<div class="collapse-title text-xl font-medium">Game Rules</div>
			<div class="collapse-content">
				<ul class="flex flex-col gap-2">
					{#each Object.entries(world_data.game_rules) as [rule, value]}
						<li class="flex flex-row p-2 bg-base-200 justify-between">
							<span>{rule}:</span><span>{value}</span>
						</li>
					{/each}
				</ul>
			</div>
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:alert" class="w-16 h-16" />
			<p class="text-lg font-semibold">No data available</p>
		</div>
	{/if}
</div>
