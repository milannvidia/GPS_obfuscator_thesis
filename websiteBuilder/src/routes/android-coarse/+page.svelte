<script lang="ts">
	import { Label, Range } from 'flowbite-svelte';
	import { base } from '$app/paths';
	const date: string[] = ['2024-10-04', '2024-10-05', '2024-10-06', '2024-10-07'];
	let date_index = $state(0);
</script>

<div class="flex h-screen flex-col">
	<div class="flex flex-row space-x-4 p-4">
		<div class="grow">
			<Label>Radius: {date[date_index]}</Label>
			<Range min="0" max={date.length - 1} step="1" bind:value={date_index}></Range>
		</div>
	</div>
	<iframe
		src="{base}/v1/{date[date_index]}.html"
		title="Map showing radius and time delta"
		class="grow"
	>
	</iframe>
</div>
