<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children?: Snippet } = $props();
</script>

<div class="flex min-h-screen w-full justify-center bg-black">
	<div
		class="relative h-screen w-[min(100vw,calc(100vh*16/9))] overflow-hidden bg-slate-950 text-slate-100"
	>
		{#if children}
			{@render children()}
		{/if}
	</div>
</div>
