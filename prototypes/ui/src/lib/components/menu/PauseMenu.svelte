<script lang="ts">
	import Button from '$lib/components/common/Button.svelte';
	import Panel from '$lib/components/common/Panel.svelte';
	import StatRow from '$lib/components/common/StatRow.svelte';

	let { onResume }: { onResume: () => void } = $props();
</script>

<div
	class="absolute inset-y-0 left-0 flex w-80 flex-col border-r-2 border-slate-800 bg-slate-950/85 p-10 backdrop-blur-sm"
>
	<div class="flex flex-1 flex-col justify-center gap-8">
		<div class="flex flex-col gap-3">
			<span class="mb-2 text-xl font-bold uppercase tracking-[0.15em] text-slate-100">
				Paused
			</span>
			<Button variant="primary" tone="menu" onclick={onResume} class="justify-start">
				Resume
			</Button>
			<Button variant="secondary" href="/beacon" class="justify-start">Abandon Run</Button>
			<Button variant="secondary" href="/menu/settings" class="justify-start">Settings</Button>
			<Button variant="secondary" href="/menu" class="justify-start">Save and Quit to Menu</Button>
			<Button variant="disabled" class="justify-start">Save and Quit Game</Button>
		</div>
		<Panel variant="panel" title="Current Expedition">
			<StatRow label="Location" value="Rusting Factory" />
			<StatRow label="Elapsed Time" value="8m 42s" />
			<StatRow label="Robot Health" value="72%" />
			<StatRow label="Cargo" value="11 / 18 slots" />
		</Panel>
	</div>

	<span class="font-mono text-[10px] text-slate-600">prototype build 0.0.1</span>
</div>
