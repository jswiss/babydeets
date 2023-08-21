<script>
	import AddBabyModal from '~root/src/lib/components/add-baby-modal.svelte';
</script>

<svelte:head>
	<title>Add Baby Details</title>
	<meta name="Add Baby Details" content="Use this form to add details about your baby" />
</svelte:head>

<section>
	<h1>Add BabyDeets</h1>

	<AddBabyModal />
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
