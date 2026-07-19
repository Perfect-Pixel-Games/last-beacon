<script lang="ts">
	import type { Snippet } from 'svelte';

	type Variant = 'primary' | 'secondary' | 'disabled';
	type Tone = 'menu' | 'beacon';

	let {
		href,
		variant = 'secondary',
		tone = 'menu',
		class: className = '',
		onclick,
		children
	}: {
		href?: string;
		variant?: Variant;
		tone?: Tone;
		class?: string;
		onclick?: () => void;
		children?: Snippet;
	} = $props();

	const toneClasses: Record<Tone, string> = {
		menu: 'bg-accent-menu border-accent-menu',
		beacon: 'bg-accent-beacon border-accent-beacon'
	};

	const variantClasses = $derived<Record<Variant, string>>({
		primary: `${toneClasses[tone]} text-slate-950 hover:brightness-110`,
		secondary: 'bg-slate-800 text-slate-100 border-slate-600 hover:bg-slate-700',
		disabled: 'bg-slate-800 text-slate-500 border-slate-700 opacity-50 pointer-events-none'
	});

	const base =
		'inline-flex items-center justify-center gap-2 rounded-sm border px-4 py-2 text-sm font-medium uppercase tracking-wide transition-colors';
</script>

{#if href && variant !== 'disabled'}
	<a {href} class="{base} {variantClasses[variant]} {className}">
		{#if children}{@render children()}{/if}
	</a>
{:else}
	<button
		type="button"
		disabled={variant === 'disabled'}
		{onclick}
		class="{base} {variantClasses[variant]} {className}"
	>
		{#if children}{@render children()}{/if}
	</button>
{/if}
