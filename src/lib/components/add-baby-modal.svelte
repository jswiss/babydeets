<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { goto } from '$app/navigation';

	let name = '';
	let sex = '';
	let birthday = '';
	let photo = '';

	async function createBaby() {
		if (name === '' || sex === '' || birthday === '') return;

		const newBaby = { name, sex, birthday, photo };

		await invoke('create_baby', { newBaby });
		goto('/');
	}
</script>

<form>
	<label for="name">
		Name
		<input name="name" bind:value={name} type="text" />
	</label>
	<label for="birthday">
		Birthday
		<input name="birthday" bind:value={birthday} type="date" />
	</label>
	<label for="sex">
		Sex
		<input name="sex" bind:value={sex} type="text" />
	</label>
	<label for="image">
		Image
		<input name="image" bind:value={photo} type="file" />
	</label>
	<button on:click={createBaby}>Submit</button>
</form>

<style>
	form {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
		flex: 0.4;
		width: max-content;
	}

	input {
		width: 90%;
	}
</style>
