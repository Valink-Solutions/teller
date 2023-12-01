<script lang="ts">
	import { onMount } from 'svelte';
	import { closeModal, modals } from 'svelte-modals';
	import { toast } from '@zerodevx/svelte-toast';
	import { canSubmit, logSubmissionTime } from '$lib/stores/feedback';

	export let isOpen: boolean;
	let stackIndex: number = $modals.length;

	let feedbackText: string = '';
	let urgency: number = 0;

	function updateUrgency(urgen: number) {
		if (urgency === urgen) {
			urgency = 0;
		} else {
			urgency = urgen;
		}
	}

	async function submitFeedback() {
		if (!$canSubmit) {
			toast.push('Please wait for 5 minutes before submitting new feedback.', {
				theme: {
					'--toastBackground': '#f44336',
					'--toastProgressBackground': '#d32f2f'
				}
			});
			return;
		}

		logSubmissionTime();

		const apiBaseUrl = 'https://feedback.chunkvault.com';
		const endpoint = `${apiBaseUrl}/feedback`;
		const response = await fetch(endpoint, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				text: feedbackText,
				urgency: urgency
			})
		});

		if (response.ok) {
			const responseData = await response.json();
			if (responseData && responseData.id) {
				toast.push(`Feedback submitted successfully!`);
			} else {
				toast.push('Feedback submitted, but no ID was returned.');
			}
			closeModal();
		} else {
			const errorMessage = await response.text();
			toast.push(`Error submitting feedback: ${errorMessage}`, {
				theme: {
					'--toastBackground': '#f44336',
					'--toastProgressBackground': '#d32f2f'
				}
			});
		}
	}
</script>

{#if isOpen}
	<div role="dialog" class="fixed inset-0 flex items-center justify-center z-50">
		<div
			class="card bg-base-100 h-fit w-full min-w-[25rem] max-w-[66.666667%] max-h-[85%] overflow-auto"
		>
			<div class="card-body gap-4">
				<h2 class="card-title">Submit Feedback</h2>
				<p>Feel free to submit suggestions or report bugs.</p>

				<form class="form-control gap-4">
					<textarea
						class="textarea w-full"
						placeholder="Enter feedback, a bug report, or a feature request here."
						bind:value={feedbackText}
					/>

					<div class="flex justify-between">
						<button
							class={urgency === 10 ? 'btn btn-primary' : 'btn btn-primary btn-outline'}
							on:click={() => updateUrgency(10)}>&#128556; Commend</button
						>
						<button
							class={urgency === 20 ? 'btn btn-primary' : 'btn btn-primary btn-outline'}
							on:click={() => updateUrgency(20)}>&#128375; Bug</button
						>
						<button
							class={urgency === 30 ? 'btn btn-primary' : 'btn btn-primary btn-outline'}
							on:click={() => updateUrgency(30)}>&#128640; Feature</button
						>
						<button
							class={urgency === 50 ? 'btn btn-error' : 'btn btn-error btn-outline'}
							on:click={() => updateUrgency(50)}>&#128163; Urgent</button
						>
					</div>
				</form>

				<div class="justify-between items-center card-actions">
					<a
						class="link text-sm"
						target="_blank"
						href="https://github.com/Valink-Solutions/teller/issues"
						>Submit more info in a github issue.</a
					>

					<div class="flex flex-row gap-2">
						{#if stackIndex > 1}
							<button on:click={closeModal} class="btn">Close</button>
						{:else}
							<button on:click={closeModal} class="btn">Close</button>
						{/if}

						<button on:click={submitFeedback} class="btn btn-primary" disabled={!$canSubmit}
							>Submit</button
						>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
