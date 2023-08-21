<script src="typescript">
	import { invoke } from '@tauri-apps/api';
	import { goto } from '$app/navigation';

	let name = '';
	let sex = '';
	let birthday = '';

	let errorMessage = '';

	async function createBaby() {
		console.log('in create');
		console.log({ name, sex, birthday });
		if (name === '' || sex === '' || birthday === '') return;

		const newBaby = { name, sex, birthday };

		await invoke('create_baby', { newBaby });
		goto('/about');
	}
</script>

<section>
	<h1>Add Deets</h1>

	<div>
		<label>
			Name
			<input name="name" bind:value={name} type="text" />
		</label>
		<label>
			Birthday
			<input name="birthday" bind:value={birthday} type="date" />
		</label>
		<label>
			Sex
			<input name="sex" bind:value={sex} type="text" />
		</label>
		<button on:click={createBaby}>Submit</button>
	</div>
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
