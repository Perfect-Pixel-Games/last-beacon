<script lang="ts">
	import { page } from '$app/state';
	import BeaconTopNav from '$lib/components/beacon/BeaconTopNav.svelte';
	import BeaconScene from '$lib/components/beacon/BeaconScene.svelte';
	import { siloLevel, siloResources } from '$lib/placeholder-data';

	let { children } = $props();

	const captions: Record<string, string> = {
		'/beacon': 'Camera: Central Overlook',
		'/beacon/hangar': 'Camera: Hangar Bay',
		'/beacon/garage': 'Camera: Garage Bay — Selected Robot Centered, Others Surrounding',
		'/beacon/missions': 'Camera: Mission Control',
		'/beacon/fabrication': 'Camera: Fabrication Bay',
		'/beacon/upgrades': 'Camera: Silo Cross-Section'
	};
	let caption = $derived(captions[page.url.pathname] ?? '');
</script>

<div class="relative flex h-full flex-col bg-slate-950">
	<BeaconScene {caption} />

	<header class="relative grid shrink-0 grid-cols-[1fr_auto_1fr] items-center gap-4 px-4 py-2">
		<div></div>

		<BeaconTopNav />

		<div class="flex items-center justify-end gap-3">
			<div
				class="flex items-center gap-2.5 rounded-sm border border-slate-700 bg-slate-900/90 px-3 py-1.5 backdrop-blur-sm"
			>
				{#each siloResources as resource (resource.name)}
					<div class="flex items-center gap-1" title={resource.name}>
						<span class="h-1.5 w-1.5 shrink-0 rounded-full {resource.dotClass}"></span>
						<span class="font-mono text-[11px] text-slate-300">{resource.amount}</span>
					</div>
				{/each}
			</div>
			<div
				class="flex items-center gap-1.5 rounded-sm border-2 border-slate-700 bg-slate-900/90 px-3 py-1.5 backdrop-blur-sm"
			>
				<span class="font-mono text-[10px] uppercase tracking-wider text-slate-500">Silo</span>
				<span class="text-sm font-bold text-accent-beacon">{siloLevel}</span>
			</div>
		</div>
	</header>
	<main class="relative flex-1 overflow-y-auto">
		{@render children()}
	</main>
</div>
