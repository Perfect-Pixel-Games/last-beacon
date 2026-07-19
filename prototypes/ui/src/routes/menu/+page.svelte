<script lang="ts">
	import Button from '$lib/components/common/Button.svelte';
	import Panel from '$lib/components/common/Panel.svelte';
	import StatRow from '$lib/components/common/StatRow.svelte';
</script>

<div class="relative h-full w-full">
	<Panel
		variant="placeholder"
		label="Save Scene — Equipped Robot in Silo / Last Location if Mid-Run"
		class="absolute inset-0 h-full w-full"
	/>

	<div
		class="absolute inset-y-0 left-0 flex w-80 flex-col border-r-2 border-slate-800 bg-slate-950/85 p-10 backdrop-blur-sm"
	>
		<div class="flex flex-1 flex-col justify-center gap-8">
			<div class="flex flex-col gap-3">
				<span class="mb-2 text-xl font-bold uppercase tracking-[0.15em] text-slate-100">
					Last Beacon
				</span>
				<Button variant="primary" tone="menu" href="/beacon" class="justify-start">Continue</Button>
				<Button variant="secondary" href="/beacon" class="justify-start">Quick Run</Button>
				<Button variant="secondary" href="/beacon" class="justify-start">New Game</Button>
				<Button variant="secondary" href="/menu/settings" class="justify-start">Settings</Button>
				<Button variant="secondary" href="/menu/credits" class="justify-start">Credits</Button>
				<Button variant="disabled" class="justify-start">Quit Game</Button>
			</div>
			<Panel variant="panel" title="Current Save">
				<StatRow label="Colony" value="The Beacon — Sector 4" />
				<StatRow label="Play Time" value="14h 22m" />
				<StatRow label="Colonists" value="12 / 15" />
				<StatRow label="Last Expedition" value="Rusting Factory — Success" />
			</Panel>
		</div>

		<span class="font-mono text-[10px] text-slate-600">prototype build 0.0.1</span>
	</div>
</div>
