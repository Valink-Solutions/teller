<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { parseDifficulty, parseGameType } from '$lib/utils';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import dayjs from 'dayjs';

	let world_data: any;

	onMount(() => {
		invoke('get_world_by_id', { worldId: $page.params.worldId })
			.then((res) => {
				world_data = res;
			})
			.catch((err) => {
				console.log(err);
				goto('/local');
			});
	});
</script>

<button class="ml-6 btn btn-ghost" on:click={() => goto('/local')}>Back</button>

{#if world_data}
	<div class="p-6 bg-gray-100">
		<div class="flex flex-row">
			<img src="" alt="" />
			<div>
				<h1 class="text-4xl font-bold mb-2">{world_data.LevelName}</h1>
				<p class="mb-2">
					Last Played: {dayjs(world_data.LastPlayed).format('MMMM D, YYYY [at] h:mm A')}
				</p>
				<p class="mb-2">Game Type: {parseGameType(world_data.GameType)}</p>
				{#if world_data.hardcore}
					<p class="mb-2">Difficulty: Hardcore</p>
				{:else}
					<p class="mb-2">Difficulty: {parseDifficulty(world_data.Difficulty)}</p>
				{/if}
			</div>
		</div>

		<!-- <h2 class="text-2xl font-bold mt-4 mb-2">Player Information</h2> -->
		<!-- <p class="mb-2">Health: {world_data.Player.Health}</p> -->
		<!-- <h2 class="text-2xl font-bold mt-4 mb-2">World Generation Settings</h2>
		<p class="mb-2">Seed: {world_data.WorldGenSettings.seed}</p>
		<div class="collapse collapse-arrow border-4 border-black drop-shadow-neu">
			<input type="checkbox" />
			<div class="collapse-title text-xl font-medium">Game Rules</div>
			<div class="collapse-content">
				<ul class="flex flex-col gap-2">
					{#each Object.entries(world_data.GameRules) as [rule, value]}
						<li class="flex flex-row p-2 bg-base-200 justify-between">
							<span>{rule}:</span><span>{value}</span>
						</li>
					{/each}
				</ul>
			</div>
		</div> -->

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
