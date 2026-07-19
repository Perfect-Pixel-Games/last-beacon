<script lang="ts">
	import PageHeading from '$lib/components/common/PageHeading.svelte';
	import Panel from '$lib/components/common/Panel.svelte';
	import UpgradeTree from '$lib/components/beacon/UpgradeTree.svelte';
	import { upgradeTree } from '$lib/placeholder-data';

	let selectedId: string | null = $state(null);
	let selectedNode = $derived(upgradeTree.find((node) => node.id === selectedId) ?? null);
</script>

<div class="relative h-full w-full">
	<div class="absolute inset-x-0 top-0 px-6 pt-6">
		<PageHeading title="Silo Upgrades" subtitle="Permanent improvements to the Beacon." />
	</div>

	<div class="absolute inset-0 p-6 pt-28">
		<div
			class="relative h-full w-full overflow-auto rounded-sm border-2 border-slate-700 bg-slate-900/90 backdrop-blur-sm"
		>
			<UpgradeTree bind:selectedId />
		</div>
	</div>

	{#if selectedNode}
		<div class="absolute right-6 top-28 w-80">
			<Panel
				variant="panel"
				title={selectedNode.name}
				class="bg-slate-900/95 shadow-xl backdrop-blur-sm"
			>
				<p class="mb-2 font-mono text-[10px] uppercase tracking-wider text-slate-500">
					{selectedNode.cost}
				</p>
				<p class="text-sm leading-relaxed text-slate-300">{selectedNode.description}</p>
			</Panel>
		</div>
	{/if}
</div>
