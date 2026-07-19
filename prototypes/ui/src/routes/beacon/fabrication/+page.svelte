<script lang="ts">
	import PageHeading from '$lib/components/common/PageHeading.svelte';
	import ModuleBrowser from '$lib/components/beacon/ModuleBrowser.svelte';
	import FloatingStatsPanel from '$lib/components/beacon/FloatingStatsPanel.svelte';
	import FeaturesPanel from '$lib/components/beacon/FeaturesPanel.svelte';
	import { moduleInventory, robotFeatures } from '$lib/placeholder-data';

	let selectedItemId: string | null = $state(null);
	let selectedItem = $derived(moduleInventory.find((item) => item.id === selectedItemId) ?? null);

	const baseStats = {
		weight: 340,
		power: 12,
		storage: 18,
		durability: 65,
		mobility: 50,
		cost: 2400
	};

	let statRows = $derived([
		{ label: 'Weight', value: `${baseStats.weight} kg`, delta: selectedItem?.weight },
		{ label: 'Power Draw', value: `${baseStats.power} kW`, delta: selectedItem?.power },
		{ label: 'Storage', value: `${baseStats.storage} slots`, delta: selectedItem?.storage },
		{
			label: 'Durability',
			control: 'slider' as const,
			sliderValue: baseStats.durability,
			delta: selectedItem?.durability
		},
		{
			label: 'Mobility',
			control: 'slider' as const,
			sliderValue: baseStats.mobility,
			delta: selectedItem?.mobility
		},
		{ label: 'Cost', value: `${baseStats.cost.toLocaleString()} scrap`, delta: selectedItem?.cost }
	]);
</script>

<div class="relative h-full w-full">
	<div class="absolute inset-x-0 top-0 px-6 pt-6">
		<PageHeading title="Fabrication" subtitle="Build or modify a robot." />
	</div>

	<div class="absolute inset-0 flex gap-4 p-6 pt-28">
		<aside
			class="w-1/3 shrink-0 overflow-hidden rounded-sm border-2 border-slate-700 bg-slate-900/90 backdrop-blur-sm"
		>
			<ModuleBrowser bind:selectedId={selectedItemId} />
		</aside>

		<div class="relative flex-1">
			<div class="absolute right-0 top-0 flex w-72 flex-col gap-4">
				<FloatingStatsPanel rows={statRows} />
				<FeaturesPanel features={robotFeatures} pendingFeature={selectedItem?.feature ?? null} />
			</div>
		</div>
	</div>
</div>
