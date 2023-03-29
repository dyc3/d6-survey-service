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

	export let response: Response | undefined = undefined;
	let responseContent = loadResponse(response);

	function loadResponse(response: Response | undefined): string {
		if (response !== undefined && response.type === 'Text') {
			return response.content.text;
		}
		return '';
	}

	$: {
		setResponse(responseContent);
	}

	function setResponse(text: string) {
		response = { type: 'Text', content: { text } };
	}

	export let errors: ValidationError[] = [];
	$: validationErrors = buildErrorMapFromFields(errors);
</script>

<Container>
	{#if required}
		<span class="required">*</span>
	{/if}

	<div class="prompt-text">
		{#if editmode}
			<TextBox bind:value={prompt} placeholder="Prompt" on:change />
			<div>
				{#each validationErrors.get('prompt') ?? [] as error}
					<ValidationErrorRenderer {error} />
				{/each}
			</div>
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div class="description-text">
		{#if editmode}
			<TextBox bind:value={description} placeholder="Description" on:change />
			<div>
				{#each validationErrors.get('description') ?? [] as error}
					<ValidationErrorRenderer {error} />
				{/each}
			</div>
			<div>
				<input type="checkbox" bind:checked={multiline} on:change /> Multiline?
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

<style lang="scss">
	@import '../ui/variables';

	.prompt-text {
		font-size: $bold-font-size;
		font-weight: bold;
		color: $color-blue;
	}

	.description-text {
		font-size: $main-font-size;
		color: $color-blue;
	}
</style>
