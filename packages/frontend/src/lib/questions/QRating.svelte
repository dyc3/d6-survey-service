<script lang="ts">
	import type { Response, ValidationError } from '$lib/common';
	import TextBox from '$lib/ui/TextBox.svelte';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';
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
	let selected: number[] = loadResponse(response);

	function loadResponse(response: Response | undefined): number[] {
		if (response !== undefined && response.type === 'Rating') {
			return [response.content.rating - 1];
		}
		return [];
	}

	$: {
		setResponse(selected);
	}

	function setResponse(rating: number[]) {
		if (rating.length === 0) {
			response = undefined;
			return;
		}
		response = { type: 'Rating', content: { rating: rating[0] + 1 } };
	}

	export let errors: ValidationError[] = [];
	$: validationErrors = buildErrorMapFromFields(errors);
</script>

<div>
	{#if required}
		<span class="required">*</span>
	{/if}

	<div class="prompt-text">
		{#if editmode}
			<span>On a scale of 1- <input bind:value={max_rating} type="number" on:change /></span>
			<span>
				where 1 is <input bind:value={minText} on:change /> and {max_rating} is
				<input bind:value={maxText} on:change />
			</span>
			<div>
				{#each validationErrors.get('max_rating') ?? [] as error}
					<ValidationErrorRenderer {error} />
				{/each}
			</div>
		{:else}
			<span>On a scale of 1-{max_rating}...</span>
		{/if}
	</div>

	{#if editmode}
		<div class="text-box-container prompt-text">
			<TextBox placeholder="Enter prompt..." bind:value={prompt} />
		</div>
		<div>
			{#each validationErrors.get('prompt') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		</div>
	{:else}
		<div>
			<span>{prompt}</span>
		</div>
	{/if}

	{#if editmode}
		<div class="text-box-container description-text">
			<TextBox placeholder="Insert description..." bind:value={description} />
		</div>
		<div>
			{#each validationErrors.get('description') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		</div>
	{:else}
		<div>
			<span>{description}</span>
		</div>
	{/if}

	<div class="buttons">
		<div style="width: max-content;">
			<ButtonGroup
				orientation="horizontal"
				size="small"
				buttons={Array.apply('', Array(max_rating)).map(function (x, i) {
					return (i + 1).toString();
				})}
				forceSelection={false}
				bind:selected
			/>
			<div class="align-rating-text">
				<span class="description-text">{minText}</span>
				<span class="description-text">{maxText}</span>
			</div>
			{#each validationErrors.get('response') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		</div>
	</div>
</div>

<style lang="scss">
	@import '../ui/variables';

	.buttons {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
	}

	.text-box-container {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
	}

	.align-rating-text {
		display: flex;
		justify-content: space-between;
	}
</style>
