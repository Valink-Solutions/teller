<script lang="ts">
	import '../../app.postcss';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import type { DirectorySettings } from '$lib/utils';
	import { currentDir } from '../../stores';

	let sideBar: HTMLElement | null = null;

	$: activeItem = $currentDir;

	let save_paths: DirectorySettings;

	let paths: string[] = [];

	invoke('get_save_folders').then((result) => {
		if (result) {
			save_paths = result as DirectorySettings;
			paths = Object.keys(save_paths.paths);
		} else {
			console.log(result);
		}
	});

	onMount(() => {
		sideBar = document.querySelector('.side-bar');
		listen('saves_config_updated', () => {
			invoke('get_save_folders').then((result) => {
				if (result) {
					save_paths = result as DirectorySettings;
					paths = Object.keys(save_paths.paths);
				} else {
					console.log(result);
				}
			});
			console.log('Event received: saves_config_updated');
		});
	});

	const toggleSideBar = () => {
		if (sideBar) {
			sideBar.classList.toggle('collapse');
		}
	};

	const handleItemClick = (item: string) => {
		currentDir.set(item);
		goto(`/local`, { replaceState: true, invalidateAll: true });
	};
</script>

<div class="flex flex-row max-h-screen max-w-screen overflow-hidden" data-name="sidebar">
	<div class="min-h-screen max-w-sm w-1/4 p-2">
		<div class="card flex flex-col h-full p-2 bg-base-100 gap-4">
			<div class="flex flex-row justify-center gap-2 items-center">
				<h1 class="font-bold">Teller</h1>
				<span class="badge badge-xs"> v0.1 </span>
			</div>

			<div class="flex flex-col h-full gap-2">
				<h2 class="w-full text-center text-xs">This Device</h2>
				<ul class="menu bg-base-200 bg-opacity-50 max-w-full rounded-sm text-xs">
					<ul class="menu bg-base-200 bg-opacity-50 max-w-full rounded-sm text-xs">
						<li>
							<button
								on:click={() => handleItemClick('default')}
								class:active={activeItem === 'default'}>Default</button
							>
						</li>
						{#each paths as path}
							<li>
								<button on:click={() => handleItemClick(path)} class:active={activeItem === path}>
									{#if path.length > 14}
										{path.slice(0, 14)}
									{:else}
										{path}
									{/if}
								</button>
							</li>
						{/each}
					</ul>
				</ul>
			</div>

			<!-- <div class="flex h-20 w-full rounded bg-black bg-opacity-10 p-2 items-center justify-center">
				<a href="/login" class="btn btn-primary btn-block">Login</a>
			</div> -->
		</div>
	</div>

	<div class="flex container max-w-full">
		<div class="flex-1 overflow-y-auto overflow-x-hidden py-2">
			<slot />
		</div>
	</div>
</div>
