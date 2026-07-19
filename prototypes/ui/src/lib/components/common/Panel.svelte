<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		variant = 'panel',
		label = '',
		title = '',
		class: className = '',
		children
	}: {
		variant?: 'placeholder' | 'panel';
		label?: string;
		title?: string;
		class?: string;
		children?: Snippet;
	} = $props();
</script>

{#if variant === 'placeholder'}
	<div
		class="relative flex min-h-16 items-center justify-center overflow-hidden rounded-sm border-2 border-dashed border-slate-600 bg-slate-800/40 {className}"
	>
		{#if label}
			<span
				class="absolute left-2 top-2 z-10 rounded-sm bg-slate-900/80 px-2 py-0.5 font-mono text-[10px] uppercase tracking-wider text-slate-400"
			>
				{label}
			</span>
		{/if}
		{#if children}
			{@render children()}
		{:else}
			<span class="font-mono text-xs text-slate-600">placeholder</span>
		{/if}
	</div>
{:else}
	<div
		class="flex flex-col overflow-hidden rounded-sm border-2 border-slate-700 bg-slate-900 {className}"
	>
		{#if title}
			<div class="border-b border-slate-700 px-4 py-2">
				<h3 class="text-xs font-semibold uppercase tracking-wider text-slate-400">{title}</h3>
			</div>
		{/if}
		<div class="flex-1 p-4">
			{#if children}
				{@render children()}
			{/if}
		</div>
	</div>
{/if}
