<script lang="ts">
	import '../../app.postcss';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import type { DirectorySettings } from '$lib/utils';
	import { currentDir, type CurrentDir } from '../../stores';

	let sideBar: HTMLElement | null = null;

	$: activeItem = $currentDir;

	let save_paths: DirectorySettings;

	let paths: string[] = [];

	invoke('get_save_folders').then((result) => {
		if (result) {
			save_paths = result as DirectorySettings;
			let tempPaths: string[] = [];
			for (let category in save_paths.categories) {
				tempPaths = Object.keys(save_paths.categories[category].paths);
			}
			paths = tempPaths;
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
					let tempPaths: string[] = [];
					for (let category in save_paths.categories) {
						tempPaths = Object.keys(save_paths.categories[category].paths);
					}
					paths = tempPaths;
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

	const handleItemClick = (item: CurrentDir) => {
		currentDir.set(item);
		goto(`/local`, { replaceState: true, invalidateAll: true });
	};
</script>

<div class="flex flex-row max-h-screen max-w-screen" data-name="sidebar">
	<div class="h-screen w-[348px] lg:w-[400px] p-2 overflow-hidden">
		<div class="card flex flex-col h-fit min-h-full p-2 bg-base-100 gap-4 overflow-hidden">
			<div class="flex flex-row justify-center gap-2 items-center">
				<h1 class="font-bold">ChunkVault</h1>
				<span class="badge badge-xs"> v0.1 </span>
			</div>

			<div class="flex flex-col h-full gap-2">
				<h2 class="w-full text-center text-xs">This Device</h2>
				<div class="max-h-[500px] overflow-hidden overflow-y-auto">
					<ul class="menu menu-vertical min-w-[190px] w-full rounded-box gap-2">
						<li>
							<button
								on:click={() => handleItemClick({ category: 'default', path: 'default' })}
								class:active={activeItem.path === 'default' && activeItem.category === 'default'}
								class="text-xs">Default</button
							>
						</li>
						{#if save_paths && save_paths.categories}
							{#each Object.keys(save_paths.categories) as category}
								<li>
									<details>
										<summary class="text-xs text-ellipsis">
											{#if category.length > 18}
												{category.slice(0, 15) + '...'}
											{:else}
												{category}
											{/if}
										</summary>
										<ul>
											{#each Object.keys(save_paths.categories[category].paths) as path}
												<li>
													<button
														on:click={() => handleItemClick({ category: category, path: path })}
														class:active={activeItem.path === path &&
															activeItem.category === category}
														class="text-xs"
													>
														{#if path.length > 18}
															{path.slice(0, 15) + '...'}
														{:else}
															{path}
														{/if}
													</button>
												</li>
											{/each}
										</ul>
									</details>
								</li>
							{/each}
						{/if}
					</ul>
				</div>
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
