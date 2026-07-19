<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children?: Snippet } = $props();
</script>

<div class="flex h-full flex-col bg-slate-950">
	<main class="flex-1 overflow-y-auto">
		{#if children}
			{@render children()}
		{/if}
	</main>
</div>
