<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import dayjs from 'dayjs';
	import { open } from '@tauri-apps/api/shell';
	let world_data: any;

	let currentPage = 1;
	const itemsPerPage = 6;

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
		} catch (err) {
			console.log(err);
			error = true;
			// goto('/local');
		} finally {
			loading = false;
		}
	});

	const handleClick = async () => {
		try {
			console.log('Opening world folder');
			await invoke('open_world_in_explorer', { worldId: $page.params.worldId });
		} catch (err) {
			console.error(err);
		}
	};
</script>

<div class="flex flex-col justify-start w-full px-4 gap-4">
	<div class="flex flex-row justify-between items-center">
		<button class="btn btn-ghost w-20" on:click={() => goto('/local')}>
			<Icon icon="mdi:arrow-left" class="w-6 h-6" />
		</button>

		<button class="btn btn-sm btn-secondary" on:click={handleClick}> Open World Folder </button>
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
		<div class="flex flex-row items-center space-x-4">
			<div class="relative w-28 h-24">
				<img
					src={world_data.icon.length > 0
						? world_data.icon
						: 'https://static.planetminecraft.com/files/image/minecraft/project/2020/194/13404399_l.jpg'}
					alt={world_data.name}
					class="object-cover w-full h-full self-start border-4 border-black shadow-neu"
				/>
				<div class="badge badge-xs absolute -bottom-2 left-0 right-0 mx-auto">
					{world_data.game_engine}
				</div>
			</div>
			<div class="flex flex-col w-full">
				<h1 class="text-4xl font-bold mb-2">{world_data.name}</h1>
				<p class="flex flex-row items-center text-sm mb-1 gap-2">
					<Icon icon="mdi:calendar-clock" class="mr-1" />
					<span class="font-semibold">Last Played:</span>
					{dayjs(world_data.last_played).format('MMMM D, YYYY [at] h:mm A')}
				</p>
				<div class="flex flex-row items-center justify-between">
					<p class="flex flex-row items-center mb-1 gap-2">
						<Icon icon="mdi:gamepad-variant" class="mr-1" />
						<span class="font-semibold">Game Type:</span>
						{world_data.game_type}
					</p>
					<p class="flex flex-row items-center mb-1 gap-2">
						<Icon icon="mdi:shield-outline" class="mr-1" />
						<span class="font-semibold">Difficulty:</span>
						{world_data.difficulty}
					</p>
				</div>
			</div>
		</div>

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
							Math.ceil(world_data.players.length / itemsPerPage),
							currentPage + 1
						))}
					class="btn btn-sm btn-primary"
					disabled={currentPage === Math.ceil(world_data.players.length / itemsPerPage)}
					>Next</button
				>
			</div>
		</div>
		<div class="grid grid-cols-2 xl:grid-cols-3 gap-4 2xl:align-start">
			{#each paginatedPlayers as player}
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
						<!-- <span class="text-xs">{player.id}</span> -->
					</div>

					<a
						class="btn btn-ghost"
						href={`/local/worlds/${$page.params.worldId}/player/${player.id}`}
					>
						<Icon icon="mdi:arrow-right" />
					</a>

					<!-- <button class="btn btn-ghost" on:click={() => handleClick(player)}>
						<Icon icon="mdi:arrow-right" />
					</button> -->
				</div>
			{/each}
		</div>

		<h1 class="border-l-4 pl-2 border-primary text-lg font-bold">World Data</h1>
		<div class="collapse collapse-arrow border-4 border-black drop-shadow-neu">
			<input type="checkbox" />
			<div class="collapse-title text-xl font-medium">Game Rules</div>
			<div class="collapse-content">
				<ul class="grid grid-cols-2 xl:grid-cols-3 gap-2">
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
