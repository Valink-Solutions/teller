<script lang="ts">
	import { closeModal, modals } from 'svelte-modals';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';

	export let isOpen: boolean;

	export let worldName: String = 'World';
	export let worldId: string;

	export let category: string | null;

	let stackIndex: number = $modals.length;

	async function backupWorld() {
		toast.push('Creating backup...');
		closeModal();
		let res = await invoke('plugin:backup_handler|create_backup_from_id', {
			worldId: worldId,
			category: category
		});
		if (res) {
			toast.push('Backup created!');
		}
		console.log(res);
	}
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50">
		<div class="card bg-slate-100 min-w-[25rem]">
			<div class="card-body">
				<h2 class="card-title">Backup {worldName}?</h2>
				<div class="justify-end card-actions">
					{#if stackIndex > 1}
						<button on:click={closeModal} class="btn">Close One</button>
					{:else}
						<button on:click={closeModal} class="btn">Close</button>
					{/if}
					<button on:click={backupWorld} class="btn btn-primary">Backup</button>
				</div>
			</div>
		</div>
	</div>
{/if}
