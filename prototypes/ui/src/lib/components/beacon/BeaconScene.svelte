<script lang="ts">
	// Stand-in for the real-time 3D silo render. In the actual game this is a single persistent
	// scene the camera moves through between Beacon screens; UI renders on top of it here too.
	let { caption = '' }: { caption?: string } = $props();
</script>

<div
	class="absolute inset-0 h-full w-full overflow-hidden bg-slate-950"
	style="background-image:
		radial-gradient(ellipse 60% 50% at 50% 42%, rgba(251,191,36,0.18), transparent 70%),
		radial-gradient(ellipse 120% 90% at 50% 120%, rgba(34,211,238,0.10), transparent 60%),
		repeating-radial-gradient(circle at 50% 42%, transparent 0, transparent 38px, rgba(148,163,184,0.06) 39px, transparent 40px);"
>
	<div
		class="absolute inset-0"
		style="background-image: repeating-linear-gradient(to bottom, transparent 0, transparent 78px, rgba(15,23,42,0.5) 79px, transparent 80px);"
	></div>
	{#if caption}
		<span
			class="absolute bottom-3 left-3 rounded-sm bg-slate-950/70 px-2 py-1 font-mono text-[10px] uppercase tracking-wider text-slate-500"
		>
			{caption}
		</span>
	{/if}
</div>
