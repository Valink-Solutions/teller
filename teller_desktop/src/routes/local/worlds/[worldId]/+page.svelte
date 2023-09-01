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
			goto('/local');
		}
	});
</script>

<button class="ml-6 btn btn-ghost" on:click={() => goto('/local')}>Back</button>

{#if world_data}
	<div class="p-6 bg-gray-100">
		<div class="flex flex-row">
			<img src="" alt="" />
			<div>
				<h1 class="text-4xl font-bold mb-2">{world_data.name}</h1>
				<p class="mb-2">
					Last Played: {dayjs(world_data.last_played).format('MMMM D, YYYY [at] h:mm A')}
				</p>
				<p class="mb-2">Game Type: {world_data.game_type}</p>
				<p class="mb-2">Difficulty: {world_data.difficulty}</p>
			</div>
		</div>

		<h2 class="text-2xl font-bold mt-4 mb-2">Player Information</h2>
		<div class="collapse collapse-arrow border-4 border-black drop-shadow-neu">
			<input type="checkbox" />
			<div class="collapse-title text-xl font-medium">Players</div>
			<div class="collapse-content">
				<div class="flex flex-col gap-4">
					{#each world_data.players as player}
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
											<p class="text-xs text-opacity-50">Level: {player.level}</p>
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
			</div>
		</div>
		<h2 class="text-2xl font-bold mt-4 mb-2">World Generation Settings</h2>
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

		<div class="collapse collapse-arrow border-4 border-black drop-shadow-neu mt-2">
			<input type="checkbox" />
			<div class="collapse-title text-xl font-medium">Raw Level.dat</div>
			<div class="collapse-content">
				<pre class="w-full p-2 bg-slate-700 text-white">
                    <code>
                        {JSON.stringify(world_data, null, 2)}
                    </code>
                </pre>
			</div>
		</div>
	</div>
{:else}
	<p>Loading world data...</p>
{/if}

<style>
	pre {
		white-space: pre-wrap;
		word-wrap: break-word;
	}
</style>
