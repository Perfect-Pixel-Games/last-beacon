<script lang="ts">
	import Panel from '$lib/components/common/Panel.svelte';
	import StatRow from '$lib/components/common/StatRow.svelte';

	type Row = {
		label: string;
		value?: string;
		control?: 'value' | 'slider';
		sliderValue?: number;
		delta?: number;
	};

	let {
		title = 'Robot Stats',
		rows
	}: {
		title?: string;
		rows: Row[];
	} = $props();
</script>

<Panel variant="panel" {title} class="bg-slate-900/95 shadow-xl backdrop-blur-sm">
	{#each rows as row (row.label)}
		<StatRow
			label={row.label}
			value={row.value}
			control={row.control}
			sliderValue={row.sliderValue}
			delta={row.delta}
		/>
	{/each}
</Panel>
