<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import ViewportFrame from '$lib/components/layout/ViewportFrame.svelte';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<ViewportFrame>
	{@render children()}
</ViewportFrame>
