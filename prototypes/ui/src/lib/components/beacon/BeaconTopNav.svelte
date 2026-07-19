<script lang="ts">
	import TabBar from '$lib/components/common/TabBar.svelte';

	const tabs = [
		{ id: 'dashboard', label: 'Dashboard', href: '/beacon' },
		{ id: 'hangar', label: 'Hangar', href: '/beacon/hangar' },
		{ id: 'garage', label: 'Garage', href: '/beacon/garage' },
		{ id: 'missions', label: 'Mission Control', href: '/beacon/missions' },
		{ id: 'fabrication', label: 'Fabrication', href: '/beacon/fabrication' },
		{ id: 'upgrades', label: 'Silo Upgrades', href: '/beacon/upgrades' }
	];
</script>

<TabBar {tabs} tone="beacon" />
