<script lang="ts">
	import Panel from '$lib/components/common/Panel.svelte';

	let {
		features,
		pendingFeature = null
	}: {
		features: string[];
		pendingFeature?: string | null;
	} = $props();
</script>

<Panel variant="panel" title="Features" class="bg-slate-900/95 shadow-xl backdrop-blur-sm">
	{#if features.length === 0 && !pendingFeature}
		<p class="text-xs text-slate-500">No special features installed.</p>
	{:else}
		<ul class="flex flex-col gap-1.5">
			{#each features as feature (feature)}
				<li class="flex items-center gap-2 text-sm text-slate-200">
					<span class="h-1.5 w-1.5 shrink-0 rounded-full bg-accent-beacon"></span>
					{feature}
				</li>
			{/each}
			{#if pendingFeature && !features.includes(pendingFeature)}
				<li class="flex items-center gap-2 text-sm text-emerald-400">
					<span class="h-1.5 w-1.5 shrink-0 rounded-full bg-emerald-400"></span>
					+ {pendingFeature}
				</li>
			{/if}
		</ul>
	{/if}
</Panel>
