<script lang="ts">
	let {
		title,
		subtitle = '',
		backHref,
		backLabel = 'Back'
	}: {
		title: string;
		subtitle?: string;
		backHref?: string;
		backLabel?: string;
	} = $props();
</script>

<div class="mb-6 flex items-center gap-3">
	{#if backHref}
		<a
			href={backHref}
			class="rounded-sm border border-slate-700 px-2 py-1 font-mono text-xs text-slate-400 transition-colors hover:border-slate-400 hover:text-slate-200"
		>
			&larr; {backLabel}
		</a>
	{/if}
	<div>
		<h1 class="text-2xl font-semibold tracking-wide text-slate-100 drop-shadow-[0_2px_6px_rgba(0,0,0,0.8)]">
			{title}
		</h1>
		{#if subtitle}
			<p class="text-xs text-slate-400 drop-shadow-[0_1px_4px_rgba(0,0,0,0.8)]">{subtitle}</p>
		{/if}
	</div>
</div>
