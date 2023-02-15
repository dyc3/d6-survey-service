<script lang="ts">
	import TextBox from '$lib/ui/TextBox.svelte';

	export let editmode = false;
	let response = 0;
	export let prompt: string;
	export let description: string;
	export let max_rating = 10;
</script>

<div>
	<div>
		{#if editmode}
			<span>On a scale of 1- <input bind:value={max_rating} type="number" /> ...</span>
		{:else}
			<span>On a scale of 1-{max_rating}...</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={prompt} />
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={description} />
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<div>
		<input bind:value={response} type="number" disabled={editmode} />
	</div>
</div>
