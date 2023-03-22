<script lang="ts">
	import type { Response } from '$lib/common';
	import TextBox from '$lib/ui/TextBox.svelte';
	import Container from '$lib/ui/Container.svelte';
	import './questions.scss';

	export let editmode = false;
	export let multiline = false;
	export let prompt: string;
	export let description: string;
	export let required = false;

	let responseContent = '';
	export let response: Response | undefined = undefined;

	$: {
		if (response !== undefined && responseContent === '') {
			if (response.type === 'Text') {
				responseContent = response.content.text;
			}
		}
		response = { type: 'Text', content: { text: responseContent } };
	}
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
	<TextBox bind:value={responseContent} disabled={editmode} {multiline} />
</div>
