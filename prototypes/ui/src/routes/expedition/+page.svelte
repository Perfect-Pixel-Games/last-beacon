<script lang="ts">
	import PauseMenu from '$lib/components/menu/PauseMenu.svelte';

	let paused = $state(false);
</script>

<svelte:window
	onkeydown={(event) => {
		if (event.key === 'Escape') paused = !paused;
	}}
/>

<div class="relative flex h-full w-full items-center justify-center bg-slate-950">
	<span class="text-4xl font-semibold uppercase tracking-widest text-slate-100">HUD</span>
	<a
		href="/beacon"
		class="absolute bottom-4 right-4 font-mono text-[10px] uppercase tracking-wider text-slate-600 hover:text-slate-400"
	>
		&larr; Return to Beacon
	</a>

	{#if paused}
		<PauseMenu onResume={() => (paused = false)} />
	{/if}
</div>
