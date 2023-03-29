<script lang="ts">
	import type { Response, ValidationError } from '$lib/common';
	import TextBox from '$lib/ui/TextBox.svelte';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';
	import Container from '$lib/ui/Container.svelte';
	import './questions.scss';
	import { buildErrorMapFromFields } from '$lib/validation';
	import ValidationErrorRenderer from '$lib/ValidationErrorRenderer.svelte';

	export let editmode = false;
	export let prompt: string;
	export let description: string;
	export let max_rating = 10;
	export let required = false;

	export let minText = 'low';
	export let maxText = 'high';

	export let response: Response | undefined = undefined;
	let group_selected: number | undefined = loadResponse(response);

	function loadResponse(response: Response | undefined): number | undefined {
		if (response !== undefined && response.type === 'Rating') {
			return response.content.rating - 1;
		}
		return undefined;
	}

	$: {
		setResponse(group_selected);
	}

	function setResponse(rating: number | undefined) {
		if (rating === undefined) {
			response = undefined;
			return;
		}
		response = { type: 'Rating', content: { rating: rating + 1 } };
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
			<span>On a scale of 1- <input bind:value={max_rating} type="number" on:change /> ...</span>
			<span
				>Where 1 is <input bind:value={minText} on:change /> and {max_rating} is
				<input bind:value={maxText} on:change /></span
			>
			{#each validationErrors.get('max_rating') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{:else}
			<span>On a scale of 1-{max_rating}...</span>
		{/if}
	</div>

	<div class="text-box-container prompt-text">
		{#if editmode}
			<TextBox placeholder="Insert prompt..." bind:value={prompt} />
			{#each validationErrors.get('prompt') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div class="text-box-container description-text">
		{#if editmode}
			<TextBox placeholder="Insert description..." bind:value={description} />
			{#each validationErrors.get('description') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<div style="width: max-content;">
		<ButtonGroup
			orientation="horizontal"
			size="small"
			buttons={Array.apply('', Array(max_rating)).map(function (x, i) {
				return (i + 1).toString();
			})}
			forceSelection={false}
			bind:selected={group_selected}
		/>
		<div class="align-rating-text">
			<span class="description-text">{minText}</span>
			<span class="description-text">{maxText}</span>
		</div>
		{#each validationErrors.get('response') ?? [] as error}
			<ValidationErrorRenderer {error} />
		{/each}
	</div>
</Container>

<style lang="scss">
	@import '../ui/variables';

	.prompt-text {
		font-weight: bold;
		font-size: $bold-font-size;
		color: $color-blue;
	}

	.text-box-container {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
	}

	.description-text {
		font-size: $main-font-size;
		color: $color-blue;
	}

	.align-rating-text {
		display: flex;
		justify-content: space-between;
	}
</style>
