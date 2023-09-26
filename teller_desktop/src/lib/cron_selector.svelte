<script>
	import { backupSettings } from '$lib/stores';

	let cronSchedule = '0 0 * * *';
	let minute = 0;
	let hour = 0;
	let day = '1';
	let selectedOption = '0 0 * * *';

	const options = [
		{ label: 'Every Hour', value: '0 * * * *' },
		{ label: 'Every 6 Hours', value: '0 */6 * * *' },
		{ label: 'Every Day', value: '0 0 * * *' },
		{ label: 'Every Other Day', value: '0 0 2-30/2 * *' },
		{ label: 'Twice a Day', value: '0 */12 * * *' },
		{ label: 'Once a Week', value: '0 0 * * 0' },
		{ label: 'Once a Month', value: '0 0 1 * *' },
		{ label: 'Custom', value: 'custom' }
	];

	function updateCronSchedule() {
		console.log(selectedOption);
		if (selectedOption === 'custom') {
			cronSchedule = `${minute} ${hour} ${day} * *`;
		} else {
			cronSchedule = selectedOption;
		}
		$backupSettings.schedule = cronSchedule;
	}

	$: selectedOption, updateCronSchedule();

	$: $backupSettings.schedule = cronSchedule;

	// $: {
	//   const matchingOption = options.find(option => option.value === $backupSettings.schedule);
	//   if (matchingOption) {
	//     selectedOption = matchingOption.value;
	//   } else {
	//     selectedOption = 'custom';
	//   }
	// }
</script>

<div class="form-control w-full gap-2">
	<div class="flex flex-col items-center gap-2 w-full">
		<select bind:value={selectedOption} class="select w-full">
			{#each options as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
		{#if selectedOption === 'custom'}
			<div class="flex flex-row w-full items-center gap-2 justify-center">
				<span> at </span>
				<input
					id="hour"
					type="number"
					min="0"
					max="23"
					bind:value={hour}
					on:input={updateCronSchedule}
					class="input"
					placeholder="(0-23)"
				/>
				<span>:</span>
				<input
					id="minute"
					type="number"
					min="0"
					max="59"
					bind:value={minute}
					on:input={updateCronSchedule}
					class="input"
					placeholder="(0-59)"
				/>
				<span> every </span>
				<input
					id="day"
					type="number"
					min="1"
					max="31"
					bind:value={day}
					on:input={updateCronSchedule}
					class="input"
					placeholder="(1-31)"
				/>
				<span class="min-w-[41px]">
					{#if Number(day) > 1}
						days
					{:else}
						day
					{/if}
				</span>
				<input type="text" bind:value={cronSchedule} class="input w-full" />
			</div>
		{/if}
	</div>
</div>
