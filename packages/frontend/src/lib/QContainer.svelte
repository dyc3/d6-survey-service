<script lang="ts">
	import type { Question, Response, ValidationError } from '$lib/common';
	import QMultipleChoice from './questions/QMultipleChoice.svelte';
	import QRating from './questions/QRating.svelte';
	import QTextInput from './questions/QTextInput.svelte';
	import { buildErrorMapFromFields, unwrapInnerErrors } from '$lib/validation';
	import ValidationErrorRenderer from './ValidationErrorRenderer.svelte';

	export let editmode = false;
	export let question: Question;
	export let required = false;
	export let response: Response | undefined = undefined;
	export let errors: ValidationError[] = [];

	// let validationErrors: Map<string, ValidationError[]> = new Map();
	// $: {
	// 	validationErrors = buildErrorMapFromFields(errors);
	// }
</script>

<div class="question-container">
	{#if question.type === 'Text'}
		<QTextInput
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:multiline={question.content.multiline}
			{editmode}
			{required}
			bind:response
			on:change
			errors={unwrapInnerErrors(errors)}
		/>
		<!-- FIXME: it shouldn't be necessary to unwrap via `validationErrors.get('question')`, its inefficient. the server needs to change how survey validation responses are built so it doesn't wrap this twice. -->
	{:else if question.type === 'Rating'}
		<QRating
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:max_rating={question.content.max_rating}
			{editmode}
			{required}
			bind:response
			on:change
			errors={unwrapInnerErrors(errors)}
		/>
	{:else if question.type == 'MultipleChoice'}
		<QMultipleChoice
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:multiple={question.content.multiple}
			bind:choices={question.content.choices}
			{editmode}
			{required}
			bind:response
			on:change
			errors={unwrapInnerErrors(errors)}
		/>
	{/if}

	{#if editmode}
		<div>
			<label for="requiredquestion">Required?</label>
			<input type="checkbox" id="requiredquestion" bind:checked={required} on:change />
		</div>
	{/if}
</div>

<style lang="scss">
	@import './ui/variables';

	.question-container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		margin: auto;
		padding-top: $large-padding;
	}
</style>
