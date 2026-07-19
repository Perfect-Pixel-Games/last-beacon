<script lang="ts">
	import PageHeading from '$lib/components/common/PageHeading.svelte';
	import RobotCard from '$lib/components/beacon/RobotCard.svelte';
	import FloatingStatsPanel from '$lib/components/beacon/FloatingStatsPanel.svelte';
	import { robotRoster } from '$lib/placeholder-data';

	let equippedId = $state(robotRoster.find((r) => r.equipped)?.id ?? robotRoster[0].id);
	let selectedRobot = $derived(robotRoster.find((r) => r.id === equippedId) ?? robotRoster[0]);
</script>

<div class="relative h-full w-full">
	<div class="absolute inset-x-0 top-0 px-6 pt-6">
		<PageHeading title="Garage" subtitle="Select a robot to equip for deployment." />
	</div>

	<div class="absolute right-4 top-4 w-72">
		<FloatingStatsPanel
			title={selectedRobot.name}
			rows={[
				{ label: 'Health', control: 'slider', sliderValue: selectedRobot.health },
				{ label: 'Weight', value: selectedRobot.weight },
				{ label: 'Power Draw', value: selectedRobot.power },
				{ label: 'Storage', value: selectedRobot.storage }
			]}
		/>
	</div>

	<div
		class="absolute inset-x-0 bottom-0 flex gap-3 overflow-x-auto border-t-2 border-slate-800 bg-slate-950/85 p-4 backdrop-blur-sm"
	>
		{#each robotRoster as robot (robot.id)}
			<RobotCard
				name={robot.name}
				health={robot.health}
				equipped={equippedId === robot.id}
				onclick={() => (equippedId = robot.id)}
			/>
		{/each}
	</div>
</div>
