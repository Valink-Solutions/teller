<script lang="ts">
	import Sortable from 'sortablejs';
	import { onMount, afterUpdate } from 'svelte';
	import Icon from '@iconify/svelte';
	import { localVaults } from './stores/settings';
	import { backupSettings } from './stores/settings';

	const updateVaultName = (oldName: string, newName: string) => {
		if (oldName !== newName) {
			localVaults.update((vaults) => {
				vaults[newName] = vaults[oldName];
				delete vaults[oldName];
				return vaults;
			});
		}
	};

	const deleteDirectory = (vault: string) => {
		localVaults.update((vaults) => {
			delete vaults[vault];
			return vaults;
		});
	};

	onMount(async function () {
		Sortable.create(document.getElementById('vaults'), {
			group: {
				name: 'vaults',
				put: false,
				pull: false
			},
			animation: 200
		});
	});

	afterUpdate(() => {
		Object.entries($localVaults).forEach(([vault, _]) => {
			const vaultElement = document.getElementById(vault);
			if (vaultElement && vaultElement.parentNode) {
				Sortable.create(vaultElement, {
					animation: 200
				});
			}
		});
	});
</script>

<div class="flex flex-col max-w-full gap-2" id="vaults">
	{#each Object.entries($localVaults) as [vault, data], i (vault)}
		<div
			class="flex flex-row gap-2 card items-center max-w-full h-fit odd:bg-slate-200 p-2 text-sm"
		>
			<button on:click={() => deleteDirectory(vault)} class="btn btn-sm btn-ghost btn-square">
				<Icon icon="mdi:close-thick" class="w-4 h-4" />
			</button>
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<span
				class="whitespace-nowrap w-24 overflow-x-auto"
				contenteditable="true"
				on:keydown={(e) => {
					if (e.key === 'Enter') {
						e.preventDefault();
						e.target.blur();
					}
				}}
				on:blur={(e) => {
					if (e.target.textContent.length > 15) {
						e.target.textContent = vault; // reset to old name if new name is too long
					} else {
						updateVaultName(vault, e.target.textContent);
					}
				}}
			>
				{vault}
			</span>
			<div
				class="overflow-x-auto overflow-y-hidden h-fit max-w-md md:max-w-lg lg:max-w-xl d-flex align-items-center justify-content-center"
			>
				<span class="w-full whitespace-nowrap px-2">{data.path}</span>
			</div>
			<div class="form-control">
				<label class="label cursor-pointer gap-2">
					<span class="label-text text-xs">Default</span>
					<input
						type="checkbox"
						class="toggle toggle-xs"
						checked={$backupSettings.default_vaults?.includes(vault) || false}
						on:change={(e) => {
							localVaults.update((vaults) => {
								vaults[vault].default = e.target.checked;
								let oldVaults = $backupSettings.default_vaults;
								oldVaults = oldVaults?.filter((v) => v !== vault) || [];
								if (e.target.checked) {
									oldVaults.push(vault);
								}
								$backupSettings.default_vaults = oldVaults;

								return vaults;
							});
						}}
					/>
				</label>
			</div>
		</div>
	{/each}
</div>
