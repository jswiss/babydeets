<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import type { Babies } from '$lib/types/babies';

	let babies: Babies = [];

	onMount(async () => {
		const res = await invoke<Babies>('list_babies');
		babies = await res;
	});
</script>

<section>
	<h1>BabyDeets</h1>

	{#if babies === undefined}
		<p>Waiting for babies...</p>
		<p>No babies yet!</p>
	{:else}
		{#each babies as baby}
			<a href="/babies/{baby.name}">{baby.name}</a>
		{/each}
	{/if}
</section>
