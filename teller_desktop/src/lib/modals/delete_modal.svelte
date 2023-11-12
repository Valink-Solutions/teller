<script lang="ts">
	import { closeModal, modals } from 'svelte-modals';

	export let isOpen: boolean;
	let stackIndex: number = $modals.length;

	export let deleteTitle: string;
	export let deleteMessage: string;

	let confirmed: boolean = false;

	export let deleteFunction: () => void;
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50">
		<div
			class="card bg-base-100 h-fit w-full min-w-[25rem] max-w-[66.666667%] max-h-[85%] overflow-auto"
		>
			<div class="card-body gap-4">
				<h2 class="card-title">{deleteTitle}</h2>
				<p>{deleteMessage}</p>

				<div class="justify-between card-actions">
					<div class="form-control">
						<label class="label cursor-pointer gap-4">
							<span class="label-text">Confirm</span>
							<input type="checkbox" class="toggle" bind:checked={confirmed} />
						</label>
					</div>
					<div class="flex flex-row gap-2">
						{#if stackIndex > 1}
							<button on:click={closeModal} class="btn">Close</button>
						{:else}
							<button on:click={closeModal} class="btn">Close</button>
						{/if}

						<button on:click={deleteFunction} class="btn btn-error" disabled={!confirmed}
							>Delete</button
						>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
