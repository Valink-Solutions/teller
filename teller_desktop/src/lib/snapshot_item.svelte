<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { SnapshotInfo } from './types/backups';
	import { formatBytes } from './utils';
	import dayjs from 'dayjs';

	export let snapshot: SnapshotInfo;

	export let vaultName: string;
	export let worldId: string;
</script>

<li class="card flex flex-row w-full bg-base-100 shadow-xl max-h-fit">
	<div class="card-body flex-row justify-between items-center">
		<div class="flex flex-row items-center gap-2">
			<div class="flex flex-row items-center gap-2 w-80 text-sm">
				<Icon icon="mdi:clock" />
				{dayjs(snapshot.created * 1000).format('MMMM D, YYYY [at] h:mm A')}
			</div>

			<div class="badge badge-primary badge-xs font-semibold whitespace-nowrap w-24 gap-1">
				<!-- <Icon icon="mdi:file" class="h-12 w-12"/> 360 -->
				{formatBytes(snapshot.size)}
			</div>
		</div>
		<div class="card-actions">
			<div class="join">
				<button class="btn btn-warning btn-sm join-item" disabled>Delete</button>
				<a
					href={`/local/vaults/${vaultName}/${worldId}/${snapshot.created}`}
					class="btn btn-sm join-item">View</a
				>
			</div>
		</div>
	</div>
</li>
