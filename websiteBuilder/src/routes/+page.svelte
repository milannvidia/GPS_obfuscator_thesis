<script lang="ts">
	import { Label, Range } from 'flowbite-svelte';
	import { base } from '$app/paths';
	const radiuses: number[] = [0, 50, 100, 200, 400, 800, 1600];
	const time_deltas = [0, 60, 300, 600, 1800, 3600];
	let radius_index = $state(0);
	let time_delta_index = $state(0);
</script>

<div class="flex h-screen flex-col">
	<div class="flex flex-row space-x-4 p-4">
		<div class="grow">
			<Label>Radius: {radiuses[radius_index]}</Label>
			<Range min="0" max={radiuses.length - 1} step="1" bind:value={radius_index}></Range>
		</div>
		<div class="grow">
			<Label>Time delta: {time_deltas[time_delta_index]}</Label>
			<Range min="0" max={time_deltas.length - 1} step="1" bind:value={time_delta_index}></Range>
		</div>
	</div>
	<iframe
		src="{base}/obfuscated/{radiuses[radius_index]}_{time_deltas[time_delta_index]}.html"
		title="Map showing radius and time delta"
		class="grow"
	>
	</iframe>
</div>
