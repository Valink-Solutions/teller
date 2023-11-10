import { writable, derived } from 'svelte/store';

const lastFeedbackTime = writable(0);
const timeLimit = 300000; // 5 minutes in milliseconds

export const canSubmit = derived(lastFeedbackTime, ($lastFeedbackTime) => {
	const now = Date.now();
	return now - $lastFeedbackTime >= timeLimit;
});

export function logSubmissionTime() {
	lastFeedbackTime.set(Date.now());
}
