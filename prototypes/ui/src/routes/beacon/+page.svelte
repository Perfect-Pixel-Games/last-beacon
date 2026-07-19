<script lang="ts">
	import Panel from '$lib/components/common/Panel.svelte';
	import StatRow from '$lib/components/common/StatRow.svelte';
	import PageHeading from '$lib/components/common/PageHeading.svelte';
</script>

<div class="relative h-full w-full">
	<div class="absolute inset-y-0 left-0 flex w-72 flex-col gap-4 overflow-y-auto p-6">
		<PageHeading title="Beacon Dashboard" subtitle="Sector 4 — Day 14" />

		<Panel variant="panel" title="Resources">
			<StatRow label="Scrap" value="1,240" />
			<StatRow label="Power" value="82 / 100" />
			<StatRow label="Components" value="36" />
			<StatRow label="Rare Cores" value="3" />
		</Panel>
		<Panel variant="panel" title="Colony Needs">
			<StatRow label="Food" value="Stable" />
			<StatRow label="Oxygen" value="Stable" />
			<StatRow label="Morale" value="Good" />
			<StatRow label="Housing" value="12 / 15" />
		</Panel>
		<Panel variant="panel" title="Equipped Robot">
			<StatRow label="Name" value="Scavenger Mk.I" />
			<StatRow label="Weight" value="340 kg" />
			<StatRow label="Power Draw" value="12 kW" />
			<StatRow label="Cargo" value="18 slots" />
		</Panel>
	</div>
</div>
