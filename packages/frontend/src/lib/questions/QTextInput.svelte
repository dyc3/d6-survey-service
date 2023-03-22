<script lang="ts">
	import TextBox from '$lib/ui/TextBox.svelte';
	import Container from '$lib/ui/Container.svelte';
	import './questions.scss';

	export let editmode = false;
	export let multiline = false;
	let response = '';
	export let prompt: string;
	export let description: string;
	export let required = false;
</script>

<Container>
	{#if required}
		<span class="required">*</span>
	{/if}

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
</Container>