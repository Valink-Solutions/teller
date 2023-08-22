<!-- <script>
	import WorldList from "$lib/world_list.svelte";
</script>


<WorldList saves_path={"/Volumes/extra ssd/Prism Instances/MainCraft/.minecraft/saves"} /> -->

<script>
	import { goto } from '$app/navigation';

	import { onMount } from 'svelte';

	onMount(() => {
		goto('/local');
	});
</script>
