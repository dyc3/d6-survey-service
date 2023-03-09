<script lang="ts">
	import TextBox from '$lib/ui/TextBox.svelte';

	export let editmode = false;
	export let multiline = false;
	let response = '';
	export let prompt: string;
	export let description: string;
</script>

<div>
	<div>
		{#if editmode}
			<TextBox bind:value={prompt} placeholder="Prompt" on:change />
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={description} placeholder="Description" on:change />
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<TextBox bind:value={response} disabled={editmode} {multiline} />
</div>
