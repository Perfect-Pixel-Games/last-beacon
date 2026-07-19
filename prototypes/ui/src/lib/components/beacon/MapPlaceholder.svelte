<script lang="ts">
	import Panel from '$lib/components/common/Panel.svelte';

	let {
		objectives = [],
		class: className = ''
	}: {
		objectives?: { title: string; status: string }[];
		class?: string;
	} = $props();

	const markers = [
		{ id: 'factory', label: 'Rusting Factory', top: '30%', left: '25%' },
		{ id: 'highway', label: 'Collapsed Highway', top: '55%', left: '45%' },
		{ id: 'solar', label: 'Solar Farm', top: '20%', left: '65%' },
		{ id: 'tower', label: 'Comms Tower', top: '70%', left: '70%' },
		{ id: 'mine', label: 'Mining Facility', top: '45%', left: '15%' }
	];

	let selected = $state('factory');
</script>

<Panel variant="placeholder" label="Surface Map" class={className}>
	<div
		class="absolute inset-0"
		style="background-image:
			repeating-radial-gradient(circle at 30% 65%, transparent 0, transparent 22px, rgba(148,163,184,0.12) 23px, transparent 24px),
			repeating-radial-gradient(circle at 72% 28%, transparent 0, transparent 28px, rgba(148,163,184,0.10) 29px, transparent 30px),
			repeating-radial-gradient(circle at 55% 80%, transparent 0, transparent 18px, rgba(148,163,184,0.10) 19px, transparent 20px);"
	></div>

	{#each markers as marker (marker.id)}
		<button
			type="button"
			onclick={() => (selected = marker.id)}
			style="top: {marker.top}; left: {marker.left};"
			class="absolute flex -translate-x-1/2 -translate-y-1/2 flex-col items-center gap-1"
		>
			<span
				class="h-3 w-3 rounded-full border-2 {selected === marker.id
					? 'border-accent-beacon bg-accent-beacon'
					: 'border-slate-400 bg-slate-700'}"
			></span>
			<span
				class="whitespace-nowrap rounded-sm bg-slate-900/80 px-1.5 py-0.5 font-mono text-[10px] {selected ===
				marker.id
					? 'text-accent-beacon'
					: 'text-slate-400'}"
			>
				{marker.label}
			</span>
		</button>
	{/each}

	{#if objectives.length > 0}
		<div class="absolute right-3 top-3 w-48 rounded-sm border border-slate-700 bg-slate-950/80 p-3">
			<h4 class="mb-2 font-mono text-[10px] uppercase tracking-wider text-slate-400">
				Current Objectives
			</h4>
			<ul class="space-y-1.5">
				{#each objectives as objective (objective.title)}
					<li class="text-xs">
						<span class="block text-slate-200">{objective.title}</span>
						<span class="block text-[10px] text-slate-500">{objective.status}</span>
					</li>
				{/each}
			</ul>
		</div>
	{/if}
</Panel>
