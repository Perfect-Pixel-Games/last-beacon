<script lang="ts">
	import { page } from '$app/state';

	type Tab = { id: string; label: string; href?: string };
	type Tone = 'menu' | 'beacon';

	let {
		tabs,
		active,
		tone = 'beacon',
		onSelect
	}: {
		tabs: Tab[];
		active?: string;
		tone?: Tone;
		onSelect?: (id: string) => void;
	} = $props();

	const activeClasses: Record<Tone, string> = {
		menu: 'border-accent-menu text-accent-menu',
		beacon: 'border-accent-beacon text-accent-beacon'
	};

	function isActive(tab: Tab): boolean {
		if (tab.href) return page.url.pathname === tab.href;
		return tab.id === active;
	}
</script>

<nav class="flex flex-wrap items-center gap-1">
	{#each tabs as tab (tab.id)}
		{#if tab.href}
			<a
				href={tab.href}
				class="border-b-2 px-4 py-3 text-xs font-semibold uppercase tracking-wider transition-colors {isActive(
					tab
				)
					? activeClasses[tone]
					: 'border-transparent text-slate-400 hover:text-slate-200'}"
			>
				{tab.label}
			</a>
		{:else}
			<button
				type="button"
				onclick={() => onSelect?.(tab.id)}
				class="border-b-2 px-4 py-3 text-xs font-semibold uppercase tracking-wider transition-colors {isActive(
					tab
				)
					? activeClasses[tone]
					: 'border-transparent text-slate-400 hover:text-slate-200'}"
			>
				{tab.label}
			</button>
		{/if}
	{/each}
</nav>
