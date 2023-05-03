<script lang="ts">
	import type { Question, Response, ValidationError } from '$lib/common';
	import QMultipleChoice from './questions/QMultipleChoice.svelte';
	import QRating from './questions/QRating.svelte';
	import QTextInput from './questions/QTextInput.svelte';
	import { unwrapInnerErrors } from '$lib/validation';

	export let editmode = false;
	export let question: Question;
	export let required = false;
	export let response: Response | undefined = undefined;
	export let errors: ValidationError[] = [];
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
			{errors}
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
		flex-direction: column;
		width: 100%;
	}
</style>
