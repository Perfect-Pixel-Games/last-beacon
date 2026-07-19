<script lang="ts">
	type Control = 'value' | 'toggle' | 'slider' | 'dropdown';
	type Layout = 'inline' | 'grid';

	let {
		label,
		value = '',
		control = 'value',
		checked = false,
		sliderValue = 50,
		delta,
		layout = 'inline'
	}: {
		label: string;
		value?: string;
		control?: Control;
		checked?: boolean;
		sliderValue?: number;
		delta?: number;
		layout?: Layout;
	} = $props();

	const deltaText = $derived(delta ? `${delta > 0 ? '+' : ''}${delta}` : '');
	const deltaClass = $derived(delta && delta > 0 ? 'text-emerald-400' : 'text-rose-400');
</script>

{#if layout === 'grid'}
	<div class="grid grid-cols-[12rem_1fr] items-center gap-4 py-2 text-sm">
		<span class="text-slate-400">{label}</span>
		{#if control === 'value'}
			<span class="inline-flex items-center gap-1.5">
				<span class="font-mono text-slate-100">{value}</span>
				{#if delta}
					<span class="font-mono text-[10px] {deltaClass}">{deltaText}</span>
				{/if}
			</span>
		{:else if control === 'toggle'}
			<span
				class="inline-flex h-5 w-9 items-center rounded-full border border-slate-600 px-0.5 {checked
					? 'justify-end bg-slate-600'
					: 'justify-start bg-slate-800'}"
			>
				<span class="h-3.5 w-3.5 rounded-full bg-slate-300"></span>
			</span>
		{:else if control === 'slider'}
			<span class="inline-flex items-center gap-1.5">
				<div class="h-1.5 w-full rounded-full bg-slate-700">
					<div class="h-1.5 rounded-full bg-slate-400" style="width: {sliderValue}%"></div>
				</div>
				{#if delta}
					<span class="font-mono text-[10px] {deltaClass}">{deltaText}</span>
				{/if}
			</span>
		{:else if control === 'dropdown'}
			<div
				class="w-full rounded-sm border border-slate-600 bg-slate-800 px-3 py-2 font-mono text-xs text-slate-300"
			>
				{value || '—'} ▾
			</div>
		{/if}
	</div>
{:else}
	<div class="flex items-center justify-between gap-4 py-2 text-sm">
		<span class="text-slate-400">{label}</span>
		{#if control === 'value'}
			<span class="inline-flex items-center gap-1.5">
				<span class="font-mono text-slate-100">{value}</span>
				{#if delta}
					<span class="font-mono text-[10px] {deltaClass}">{deltaText}</span>
				{/if}
			</span>
		{:else if control === 'toggle'}
			<span
				class="inline-flex h-5 w-9 items-center rounded-full border border-slate-600 px-0.5 {checked
					? 'justify-end bg-slate-600'
					: 'justify-start bg-slate-800'}"
			>
				<span class="h-3.5 w-3.5 rounded-full bg-slate-300"></span>
			</span>
		{:else if control === 'slider'}
			<span class="inline-flex items-center gap-1.5">
				<div class="h-1.5 w-32 rounded-full bg-slate-700">
					<div class="h-1.5 rounded-full bg-slate-400" style="width: {sliderValue}%"></div>
				</div>
				{#if delta}
					<span class="font-mono text-[10px] {deltaClass}">{deltaText}</span>
				{/if}
			</span>
		{:else if control === 'dropdown'}
			<span
				class="rounded-sm border border-slate-600 bg-slate-800 px-2 py-1 font-mono text-xs text-slate-300"
			>
				{value || '—'} ▾
			</span>
		{/if}
	</div>
{/if}
