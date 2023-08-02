<script>
	import { babyName, babyBirthday } from '$lib/stores';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Baby Details" />
</svelte:head>

<section>
	{#if $babyName}
		<h1>{babyName}</h1>
	{:else}
		<form method="POST">
			<label for="name">
				What's your baby's name?
				<input type="text" name="name" />
			</label>
			<label for="sex">
				What sex were they born?
				<select name="sex">
					<option value="male">Male</option>
					<option value="female">Female</option>
					<option value="other">Other</option>
				</select>
			</label>
			<label for="birthday">
				What is their birthday
				<input type="date" name="birthday" />
			</label>
			<button>Submit</button>
		</form>
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
	form {
		display: flex;
		flex-direction: column;
		flex-grow: 0.2;
		justify-content: space-around;
	}
</style>
