<script lang="ts">
	import PageHeading from '$lib/components/common/PageHeading.svelte';
	import Panel from '$lib/components/common/Panel.svelte';
	import StatRow from '$lib/components/common/StatRow.svelte';
	import { missions as missionSeed, type Mission, type MissionCategory } from '$lib/placeholder-data';

	let localMissions = $state<Mission[]>(missionSeed.map((mission) => ({ ...mission })));

	let sectionExpanded = $state<Record<MissionCategory, boolean>>({
		main: true,
		side: true,
		passive: true
	});

	let selectedId = $state(localMissions.find((m) => m.active)?.id ?? (localMissions[0]?.id ?? ''));
	let selectedMission = $derived(localMissions.find((m) => m.id === selectedId));

	function missionsIn(category: MissionCategory) {
		return localMissions.filter((m) => m.category === category);
	}

	function toggleMission(id: string) {
		const mission = localMissions.find((m) => m.id === id);
		if (!mission) return;

		if (mission.category === 'passive') {
			mission.active = !mission.active;
			return;
		}

		const activating = !mission.active;
		if (activating) {
			for (const m of localMissions) {
				if (m.category !== 'passive') m.active = false;
			}
		}
		mission.active = activating;
	}

	const sections: { key: MissionCategory; label: string }[] = [
		{ key: 'main', label: 'Main Mission' },
		{ key: 'side', label: 'Side Missions' },
		{ key: 'passive', label: 'Passive Missions' }
	];
</script>

<div class="relative h-full w-full">
	<div class="absolute inset-x-0 top-0 px-6 pt-6">
		<PageHeading title="Mission Control" subtitle="Select objectives for the next run." />
	</div>

	<div class="absolute inset-0 flex gap-4 px-6 pb-6 pt-28">
		<div
			class="flex w-80 shrink-0 flex-col overflow-y-auto rounded-sm border-2 border-slate-700 bg-slate-900/90 backdrop-blur-sm"
		>
			{#each sections as section (section.key)}
				<div class="border-b border-slate-700 last:border-b-0">
					<button
						type="button"
						onclick={() => (sectionExpanded[section.key] = !sectionExpanded[section.key])}
						class="flex w-full items-center justify-between px-3 py-2 text-left"
					>
						<span class="text-xs font-semibold uppercase tracking-wider text-slate-400">
							{section.label}
						</span>
						<span class="font-mono text-xs text-slate-500">
							{sectionExpanded[section.key] ? '−' : '+'}
						</span>
					</button>
					{#if sectionExpanded[section.key]}
						<div class="flex flex-col gap-0.5 px-2 pb-2">
							{#each missionsIn(section.key) as mission (mission.id)}
								<div
									role="button"
									tabindex="0"
									onclick={() => (selectedId = mission.id)}
									onkeydown={(event) => {
										if (event.key === 'Enter' || event.key === ' ') selectedId = mission.id;
									}}
									class="flex cursor-pointer items-center gap-2 rounded-sm px-2 py-1.5 transition-colors {selectedId ===
									mission.id
										? 'bg-accent-beacon/15'
										: 'hover:bg-slate-800/60'}"
								>
									<button
										type="button"
										aria-pressed={mission.active}
										onclick={(event) => {
											event.stopPropagation();
											toggleMission(mission.id);
											selectedId = mission.id;
										}}
										class="flex h-4 w-4 shrink-0 items-center justify-center border-2 {section.key ===
										'passive'
											? 'rounded-sm'
											: 'rounded-full'} {mission.active
											? 'border-accent-beacon'
											: 'border-slate-500'}"
									>
										{#if mission.active}
											<span
												class="h-2 w-2 bg-accent-beacon {section.key === 'passive'
													? 'rounded-[1px]'
													: 'rounded-full'}"
											></span>
										{/if}
									</button>
									<span
										class="flex-1 truncate text-sm {mission.active
											? 'text-slate-100'
											: 'text-slate-400'}"
									>
										{mission.title}
									</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		</div>

		<div class="min-w-0 flex-1 overflow-y-auto">
			<Panel variant="panel" title={selectedMission?.title ?? 'No Mission Selected'} class="h-full">
				{#if selectedMission}
					<StatRow label="Given By" value={selectedMission.giver} />
					<StatRow label="Reward" value={selectedMission.reward} />
					<p class="mb-2 mt-4 font-mono text-[10px] uppercase tracking-wider text-slate-500">
						Description
					</p>
					<p class="text-sm leading-relaxed text-slate-300">{selectedMission.description}</p>
				{:else}
					<p class="text-sm text-slate-500">Select a mission from the list to view its details.</p>
				{/if}
			</Panel>
		</div>
	</div>
</div>
