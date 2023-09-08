<script lang="ts">
	import { page } from '$app/stores';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import InventoryViewer from '$lib/inventory_viewer.svelte';
	import { goto } from '$app/navigation';

	let world_data: any;
	let player: any;
	let player_data: any;

	let loading = true;
	let error = false;

	onMount(async () => {
		try {
			const world_res = await invoke('get_world_by_id', {
				worldId: $page.params.worldId,
				returnPath: true
			});
			world_data = world_res;

			if (world_data) {
				const player_res = await invoke('grab_player_from_uuid', {
					playerUuid: $page.params.playerId,
					path: world_data
				});
				player = player_res;

				invoke('grab_player_meta_from_uuid', { playerUuid: $page.params.playerId }).then((res) => {
					player_data = res;
				});
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
	<button class="btn btn-ghost w-20" on:click={() => goto(`/local/worlds/${$page.params.worldId}`)}>
		<Icon icon="mdi:arrow-left" class="w-6 h-6" />
	</button>

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
		<div class="flex flex-col gap-4">
			<div class="flex flex-row justify-between items-center">
				<div class="flex flex-row items-center gap-2">
					<img
						src={player_data
							? player_data.avatar
							: 'https://crafthead.net/avatar/8667ba71b85a4004af54457a9734eed7?scale=32&overlay=false'}
						alt={player_data ? player_data.username : 'Default Icon'}
						class="w-8 h-8 mr-2"
					/>
					<h1 class="card-title">{player_data ? player_data.username : 'Player'}</h1>

					<h3 class="text-xs opacity-50">{player.id}</h3>
				</div>

				<div class="badge badge-primary">
					{player.game_mode === 0
						? 'Survival'
						: player.game_mode === 1
						? 'Creative'
						: player.game_mode === 2
						? 'Adventure'
						: player.game_mode === 3
						? 'Spectator'
						: player.game_mode === 5
						? 'Default'
						: 'Unknown'}
				</div>
			</div>
			<div class="flex flex-col gap-4 max-w-xl mx-auto">
				{#if player.health}
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
				{/if}
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
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center w-full h-full">
			<Icon icon="mdi:alert" class="w-16 h-16" />
			<p class="text-lg font-semibold">No data available</p>
		</div>
	{/if}
</div>
