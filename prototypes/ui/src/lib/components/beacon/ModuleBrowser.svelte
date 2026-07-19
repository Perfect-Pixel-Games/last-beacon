<script lang="ts">
	import TabBar from '$lib/components/common/TabBar.svelte';
	import { moduleCategories, moduleInventory } from '$lib/placeholder-data';

	let { selectedId = $bindable(null) }: { selectedId?: string | null } = $props();

	let activeCategory = $state(moduleCategories[0]);

	let items = $derived(moduleInventory.filter((item) => item.category === activeCategory));

	const tabs = moduleCategories.map((category) => ({ id: category, label: category }));

	function selectItem(id: string) {
		selectedId = selectedId === id ? null : id;
	}
</script>

<div class="flex h-full flex-col overflow-hidden">
	<div class="border-b border-slate-700 px-3 py-2">
		<h3 class="text-xs font-semibold uppercase tracking-wider text-slate-400">Modules</h3>
	</div>

	<div class="border-b border-slate-700 px-2">
		<TabBar {tabs} active={activeCategory} tone="beacon" onSelect={(id) => (activeCategory = id)} />
	</div>

	<div class="flex-1 overflow-y-auto p-3">
		{#if items.length === 0}
			<p class="p-2 text-xs text-slate-500">No items in storage for this category.</p>
		{:else}
			<div class="grid grid-cols-3 gap-2">
				{#each items as item (item.id)}
					<button
						type="button"
						onclick={() => selectItem(item.id)}
						class="relative flex aspect-square flex-col justify-end overflow-hidden rounded-sm border-2 bg-slate-800/40 p-1.5 text-left transition-colors {selectedId ===
						item.id
							? 'border-accent-beacon'
							: 'border-dashed border-slate-600 hover:border-slate-400'}"
					>
						{#if item.feature}
							<span
								class="absolute right-1 top-1 h-1.5 w-1.5 rounded-full bg-emerald-400"
								title="Grants a feature"
							></span>
						{/if}
						<span class="truncate text-[9px] uppercase tracking-wide text-slate-500">
							{item.type}
						</span>
						<span class="truncate text-[10px] font-medium text-slate-200">{item.name}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>
