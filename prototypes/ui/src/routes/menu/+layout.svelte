<script lang="ts">
	import MenuChrome from '$lib/components/menu/MenuChrome.svelte';

	let { children } = $props();
</script>

<MenuChrome>
	{@render children()}
</MenuChrome>
