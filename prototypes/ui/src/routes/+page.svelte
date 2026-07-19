<script lang="ts">
	import { goto } from '$app/navigation';

	function enter() {
		goto('/menu');
	}
</script>

<svelte:window
	onkeydown={(e) => {
		if (e.key === 'Enter') enter();
	}}
/>

<div class="flex h-full w-full flex-col items-center justify-center gap-6 bg-slate-950">
	<div class="flex flex-col items-center gap-2">
		<span class="font-mono text-xs uppercase tracking-[0.3em] text-slate-500">
			Perfect Pixel Games presents
		</span>
		<h1 class="text-6xl font-bold tracking-wide text-slate-100">LAST BEACON</h1>
		<p class="text-sm text-slate-500">Build. Venture. Recover. Return.</p>
	</div>
	<button
		type="button"
		onclick={enter}
		class="mt-10 animate-pulse font-mono text-sm uppercase tracking-widest text-accent-menu"
	>
		Press Enter to Continue
	</button>
</div>
