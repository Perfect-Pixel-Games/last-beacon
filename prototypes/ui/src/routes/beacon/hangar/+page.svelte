<script lang="ts">
	import PageHeading from '$lib/components/common/PageHeading.svelte';
	import Button from '$lib/components/common/Button.svelte';
	import MapPlaceholder from '$lib/components/beacon/MapPlaceholder.svelte';
	import { missions } from '$lib/placeholder-data';

	const activeObjectives = missions
		.filter((mission) => mission.active)
		.map((mission) => ({
			title: mission.title,
			status: mission.category.charAt(0).toUpperCase() + mission.category.slice(1)
		}));
</script>

<div class="relative h-full w-full">
	<div class="absolute inset-x-0 top-0 px-6 pt-6">
		<PageHeading title="Hangar" subtitle="Select a deployment site." />
	</div>

	<div class="absolute inset-0 flex flex-col items-center justify-center gap-4 px-6 pb-6 pt-28">
		<MapPlaceholder objectives={activeObjectives} class="aspect-video w-[80%] max-w-6xl" />
		<div class="flex w-[80%] max-w-6xl justify-end">
			<Button variant="primary" tone="beacon" href="/expedition">Launch Expedition</Button>
		</div>
	</div>
</div>
