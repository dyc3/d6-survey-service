<script lang="ts">
	import type { Question } from './common';
	import QMultipleChoice from './questions/QMultipleChoice.svelte';
	import QRating from './questions/QRating.svelte';
	import QTextInput from './questions/QTextInput.svelte';

	export let editmode = false;
	export let question: Question;
	export let required = false;
</script>

<div class="question-container">
	{#if question.type === 'Text'}
		<QTextInput
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:multiline={question.content.multiline}
			{editmode}
			{required}
			on:change
		/>
	{:else if question.type === 'Rating'}
		<QRating
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:max_rating={question.content.max_rating}
			{editmode}
			{required}
			on:change
		/>
	{:else if question.type == 'MultipleChoice'}
		<QMultipleChoice
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:multiple={question.content.multiple}
			bind:choices={question.content.choices}
			{editmode}
			{required}
			on:change
		/>
	{/if}

	{#if editmode}
	<div>
		<label for="requiredquestion">Required?</label>
		<input type="checkbox" id="requiredquestion" bind:checked={required} />
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
