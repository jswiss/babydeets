<svelte:head>
	<title>Add Baby Details</title>
	<meta name="Add Baby Details" content="Use this form to add details about your baby" />
</svelte:head>

<section>
	<h1>Add Deets</h1>

	<form method="POST">
		<label>
			Date
			<input name="date" type="date" />
		</label>
		<label>
			Notes
			<input name="notes" type="textarea" />
		</label>
		<button>Submit</button>
	</form>
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
