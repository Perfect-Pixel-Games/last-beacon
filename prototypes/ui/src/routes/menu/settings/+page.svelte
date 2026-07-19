<script lang="ts">
	import PageHeading from '$lib/components/common/PageHeading.svelte';
	import TabBar from '$lib/components/common/TabBar.svelte';
	import Panel from '$lib/components/common/Panel.svelte';
	import StatRow from '$lib/components/common/StatRow.svelte';
	import BeaconScene from '$lib/components/beacon/BeaconScene.svelte';

	type SettingRow = {
		label: string;
		control: 'value' | 'toggle' | 'slider' | 'dropdown';
		value?: string;
		checked?: boolean;
		sliderValue?: number;
	};

	type SettingGroup = {
		label: string;
		rows: SettingRow[];
	};

	const tabs = [
		{ id: 'accessibility', label: 'Accessibility' },
		{ id: 'gameplay', label: 'Gameplay' },
		{ id: 'video', label: 'Video' },
		{ id: 'graphics', label: 'Graphics' },
		{ id: 'audio', label: 'Audio' }
	];

	const settingsByTab: Record<string, SettingGroup[]> = {
		accessibility: [
			{
				label: 'Visual',
				rows: [
					{ label: 'Colourblind Mode', control: 'dropdown', value: 'Off' },
					{ label: 'Reduce Motion', control: 'toggle', checked: false },
					{ label: 'UI Scale', control: 'slider', sliderValue: 60 }
				]
			},
			{
				label: 'Audio & Text',
				rows: [{ label: 'Subtitles', control: 'toggle', checked: true }]
			}
		],
		gameplay: [
			{
				label: 'Difficulty',
				rows: [
					{ label: 'Difficulty', control: 'dropdown', value: 'Standard' },
					{ label: 'Auto-Extract Warning', control: 'toggle', checked: true },
					{ label: 'Push-Your-Luck Prompts', control: 'toggle', checked: true }
				]
			},
			{
				label: 'Controls',
				rows: [{ label: 'Camera Sensitivity', control: 'slider', sliderValue: 45 }]
			}
		],
		video: [
			{
				label: 'Display',
				rows: [
					{ label: 'Display Mode', control: 'dropdown', value: 'Fullscreen' },
					{ label: 'Resolution', control: 'dropdown', value: '1920 x 1080' }
				]
			},
			{
				label: 'Performance',
				rows: [
					{ label: 'V-Sync', control: 'toggle', checked: true },
					{ label: 'Frame Rate Cap', control: 'dropdown', value: '144' }
				]
			}
		],
		graphics: [
			{
				label: 'Quality',
				rows: [
					{ label: 'Quality Preset', control: 'dropdown', value: 'High' },
					{ label: 'Shadows', control: 'slider', sliderValue: 70 }
				]
			},
			{
				label: 'Effects',
				rows: [
					{ label: 'Ambient Occlusion', control: 'toggle', checked: true },
					{ label: 'Motion Blur', control: 'toggle', checked: false }
				]
			}
		],
		audio: [
			{
				label: 'Master',
				rows: [{ label: 'Master Volume', control: 'slider', sliderValue: 80 }]
			},
			{
				label: 'Channels',
				rows: [
					{ label: 'Music', control: 'slider', sliderValue: 65 },
					{ label: 'SFX', control: 'slider', sliderValue: 75 },
					{ label: 'Ambience', control: 'slider', sliderValue: 50 }
				]
			}
		]
	};

	let activeTab = $state('gameplay');
	let activeGroups = $derived(settingsByTab[activeTab] ?? []);
</script>

<div class="relative h-full w-full">
	<BeaconScene caption="Camera: Command Deck" />

	<div class="relative flex h-full flex-col p-10">
		<PageHeading title="Settings" subtitle="Simple, clean, elegant." backHref="/menu" />
		<TabBar {tabs} active={activeTab} tone="menu" onSelect={(id) => (activeTab = id)} />

		<div class="mt-6 min-h-0 max-w-xl flex-1">
			<Panel variant="panel" class="h-full">
				<div class="flex flex-col gap-5">
					{#each activeGroups as group (group.label)}
						<div>
							<h4
								class="mb-1 border-b border-slate-800 pb-1 font-mono text-[10px] uppercase tracking-wider text-slate-500"
							>
								{group.label}
							</h4>
							<div class="divide-y divide-slate-800/60">
								{#each group.rows as row (row.label)}
									<StatRow
										layout="grid"
										label={row.label}
										control={row.control}
										value={row.value}
										checked={row.checked}
										sliderValue={row.sliderValue}
									/>
								{/each}
							</div>
						</div>
					{/each}
				</div>
			</Panel>
		</div>
	</div>
</div>
