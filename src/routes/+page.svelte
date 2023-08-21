<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';

	type Baby = {
		name: String;
		sex: String;
		birthday: String;
		created_at: String;
	};

	type Babies = Array<Baby>;

	let babies: Babies = [];

	onMount(async () => {
		const res = await invoke<Babies>('list_babies');
		babies = await res;
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="All babies" content="Show all babies" />
</svelte:head>

<section>
	<h1>BabyDeets</h1>

	{#if babies === undefined}
		<p>Waiting for babies...</p>
		<p>No babies yet!</p>
	{:else}
		{#each babies as baby}
			<p>{baby.name}</p>
		{/each}
	{/if}
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}
</style>
