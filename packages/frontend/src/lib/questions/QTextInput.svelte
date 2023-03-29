<script lang="ts">
	import type { Response, ValidationError } from '$lib/common';
	import TextBox from '$lib/ui/TextBox.svelte';
	import Container from '$lib/ui/Container.svelte';
	import './questions.scss';
	import { buildErrorMapFromFields } from '$lib/validation';
	import ValidationErrorRenderer from '$lib/ValidationErrorRenderer.svelte';

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

	export let errors: ValidationError[] = [];
	$: validationErrors = buildErrorMapFromFields(errors);
</script>

<Container>
	{#if required}
		<span class="required">*</span>
	{/if}

	<div>
		{#if editmode}
			<TextBox bind:value={prompt} placeholder="Prompt" on:change />
			{#each validationErrors.get('prompt') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{:else}
			<span>{prompt}</span>
		{/if}
		
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={description} placeholder="Description" on:change />
			{#each validationErrors.get('description') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
			<div>
				<input type=checkbox bind:checked={multiline} on:change> Multiline?
			</div>
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<TextBox bind:value={responseContent} disabled={editmode} {multiline} />
	{#each validationErrors.get('response') ?? [] as error}
		<ValidationErrorRenderer {error} />
	{/each}
</Container>
