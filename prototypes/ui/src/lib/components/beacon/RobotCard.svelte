<script lang="ts">
	let {
		name,
		health,
		equipped = false,
		onclick
	}: {
		name: string;
		health: number;
		equipped?: boolean;
		onclick?: () => void;
	} = $props();

	const healthColor = $derived(health >= 70 ? 'bg-emerald-500' : health >= 35 ? 'bg-amber-500' : 'bg-rose-500');
</script>

<button
	type="button"
	{onclick}
	class="relative flex aspect-square w-36 shrink-0 flex-col justify-end overflow-hidden rounded-sm border-2 bg-slate-800/40 text-left transition-colors {equipped
		? 'border-accent-beacon'
		: 'border-dashed border-slate-600 hover:border-slate-400'}"
>
	<span
		class="absolute left-2 top-2 font-mono text-[10px] uppercase tracking-wider text-slate-500"
	>
		Robot
	</span>
	{#if equipped}
		<span
			class="absolute right-2 top-2 rounded-sm bg-accent-beacon px-1.5 py-0.5 font-mono text-[9px] font-semibold uppercase text-slate-950"
		>
			Equipped
		</span>
	{/if}
	<div class="bg-slate-900/90 p-2">
		<p class="truncate text-xs font-semibold text-slate-100">{name}</p>
		<div class="mt-1 flex items-center gap-1.5">
			<div class="h-1.5 flex-1 rounded-full bg-slate-700">
				<div class="h-1.5 rounded-full {healthColor}" style="width: {health}%"></div>
			</div>
			<span class="font-mono text-[9px] text-slate-400">{health}%</span>
		</div>
	</div>
</button>
