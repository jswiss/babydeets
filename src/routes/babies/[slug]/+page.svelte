<script lang="ts">
	let files: Array<File> = [];

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		files = Array.from(event.dataTransfer?.files || []);
	}

	async function handleFormSubmit(event: Event) {
		event.preventDefault();

		const formData = new FormData();
		files.forEach((file) => formData.append('files', file));

		try {
			const response = await fetch('http://localhost:4000/upload_files', {
				method: 'POST',
				body: formData
			});

			if (response.ok) {
				alert('Files uploaded successfully!');
			} else {
				alert('Error uploading files. Please try again later.');
			}
		} catch (error) {
			console.error('Error:', error);
			alert('An error occurred. Please try again later.');
		}
	}
</script>

<main on:drop={handleDrop} on:dragover|preventDefault>
	<h1>Drag & Drop Images</h1>
	<form on:submit={handleFormSubmit}>
		<input
			type="file"
			multiple
			on:change={(e) => {
				console.log(e);
				// @ts-ignore
				files = Array.from(e.target?.files || []);
			}}
		/>
		<button type="submit" disabled={files.length === 0}>Upload</button>
	</form>
	{#if files.length > 0}
		<h2>Selected Files:</h2>
		<ul>
			{#each files as file (file.name)}
				<li>{file.name}</li>
			{/each}
		</ul>
	{/if}
</main>
