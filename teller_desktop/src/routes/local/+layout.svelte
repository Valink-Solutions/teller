<script lang="ts">
	import '../../app.postcss';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { listen } from '@tauri-apps/api/event';
	import type { BackupSettings, DirectorySettings } from '$lib/utils';
	import { currentDir, type CurrentDir } from '$lib/stores';
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import { Modals, closeModal, openModal } from 'svelte-modals';
	import DirectoriesModal from '$lib/modals/directories_modal.svelte';
	import SettingsModal from '$lib/modals/settings_modal.svelte';

	let sideBar: HTMLElement | null = null;

	$: activeItem = $currentDir;

	let save_paths: DirectorySettings;

	let paths: string[] = [];

	let localVaults: Record<string, string>;

	invoke('plugin:config|get_save_folders').then((result) => {
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

	invoke('plugin:config|get_backup_settings').then((result) => {
		if (result) {
			let backupSettings = result as BackupSettings;
			localVaults = backupSettings.vaults as Record<string, string>;
		} else {
			console.log(result);
		}
	});

	onMount(() => {
		sideBar = document.querySelector('.side-bar');
		listen('saves_config_updated', () => {
			invoke('plugin:config|get_save_folders').then((result) => {
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

	async function handleItemClick(item: CurrentDir) {
		if (item?.type === 'world') {
			currentDir.set(item);
			await goto(`/local/worlds/${item.category}/${item.path}`);
		} else if (item?.type === 'localBackup') {
			currentDir.set(item);
			await goto(`/local/vaults`);
		}
	}

	function handleEditDirClick() {
		openModal(DirectoriesModal);
	}

	function handleSettingsClick() {
		openModal(SettingsModal);
	}

	const options = {
		theme: {
			// '--toastBackground': '#1f2937',
			// '--toastProgressBackground': '#fff',
			// '--toastProgressFill': '#1f2937',
			// '--toastColor': '#fff'
		}
	};

	let activeTab = 'local';

	const switchTab = (tab: string) => {
		activeTab = tab;
	};
</script>

<div class="flex flex-row max-h-screen max-w-screen" data-name="sidebar">
	<div class="h-screen min-w-[270px] w-[348px] lg:w-[400px] p-2 overflow-hidden relative">
		<div class="card flex flex-col h-fit min-h-full p-2 bg-base-100 gap-4 overflow-hidden">
			<div class="flex flex-row justify-center gap-2 items-center">
				<h1 class="font-bold">ChunkVault</h1>
				<span class="badge badge-xs"> v0.1 </span>
			</div>

			<button on:click={handleSettingsClick} class="absolute top-3 right-3 opacity-50">
				<Icon icon="mdi:settings" />
			</button>

			<div class="flex flex-col h-full gap-2">
				<div class="flex flex-row gap-2 items-center w-full justify-center">
					<h2 class="text-center text-xs">Instances</h2>
					<!-- <button on:click={handleEditDirClick}>
						<Icon icon="mdi:pencil" />
					</button> -->
				</div>
				<div class="max-h-[350px] overflow-hidden overflow-y-auto">
					<ul class="menu menu-vertical min-w-[190px] w-full gap-2">
						<li>
							<button
								on:click={() =>
									handleItemClick({ type: 'world', category: 'default', path: 'default' })}
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
														on:click={() =>
															handleItemClick({ type: 'world', category: category, path: path })}
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

			<div class="flex flex-col h-full gap-2">
				<div class="flex flex-row gap-2 items-center w-full justify-center">
					<h2 class="text-center text-xs">Vaults</h2>
					<!-- <button on:click={handleEditDirClick}>
						<Icon icon="mdi:pencil" />
					</button> -->
				</div>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<!-- svelte-ignore a11y-missing-attribute -->
				<div class="tabs w-full justify-center">
					<a
						class="tab {activeTab === 'local' ? 'tab-active' : ''}"
						on:click={() => switchTab('local')}>Local</a
					>
					<a class="tab tab-disabled {activeTab === 'remote' ? 'tab-active' : ''}">Remote</a>
				</div>
				<div class="tab-content">
					{#if activeTab === 'local'}
						<!-- Local Vaults content goes here -->
						{#if localVaults}
							<ul class="menu menu-vertical min-w-[190px] w-full gap-2">
								{#each Object.entries(localVaults) as [vault, path], i (vault)}
									<li>
										<button
											on:click={() =>
												handleItemClick({ type: 'localBackup', category: vault, path: path })}
											class:active={activeItem.path === path && activeItem.category === vault}
											class="text-xs"
										>
											{#if vault.length > 18}
												{vault.slice(0, 15) + '...'}
											{:else}
												{vault}
											{/if}
										</button>
									</li>
								{/each}
							</ul>
						{/if}
					{:else if activeTab === 'remote'}
						<!-- Remote Vaults content goes here -->
					{/if}
				</div>
			</div>
		</div>
	</div>

	<div class="flex container max-w-full">
		<div class="flex-1 overflow-y-auto overflow-x-hidden py-2">
			<slot />
		</div>
	</div>
</div>
<Modals>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div slot="backdrop" class="backdrop" on:click={closeModal} />
</Modals>
<SvelteToast {options} />

<style>
	:root {
		--toastContainerTop: auto;
		--toastContainerBottom: 1.5rem;
		--toastWidth: 20rem;
		--toastBackground: rgb(244, 244, 244);
		--toastColor: #000000;
		--toastBoxShadow: 0px 4px 0px 0px rgb(0, 0, 0);
		--toastBorder: 4px solid rgb(0, 0, 0);
		--toastBorderRadius: 0;
		--toastMsgPadding: 0.75rem 0.5rem;
		--toastBtnWidth: 2rem;
		--toastBtnHeight: 100%;
		--toastBtnFont: 1rem 'Martian Mono', monospace;
		--toastBarBackground: rgb(71, 213, 109);
		--toastBarTop: auto;
		--toastBarRight: auto;
		--toastBarBottom: 0;
		--toastBarLeft: 0;
		--toastBarHeight: 6px;
		--toastBarWidth: 100%;
	}
	.backdrop {
		position: fixed;
		top: 0;
		bottom: 0;
		right: 0;
		left: 0;
		background: rgba(0, 0, 0, 0.5);
	}
</style>
