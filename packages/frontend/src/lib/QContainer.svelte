<script lang="ts">
	import type { Question } from './common';
	import QMultipleChoice from './questions/QMultipleChoice.svelte';
	import QRating from './questions/QRating.svelte';
	import QTextInput from './questions/QTextInput.svelte';

	export let editmode = false;
	export let question: Question;
</script>

<div class="question-container">
	{#if question.type === 'Text'}
		<QTextInput
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:multiline={question.content.multiline}
			{editmode}
			on:change
		/>
	{:else if question.type === 'Rating'}
		<QRating
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:max_rating={question.content.max_rating}
			{editmode}
			on:change
		/>
	{:else if question.type == 'MultipleChoice'}
		<QMultipleChoice
			bind:prompt={question.content.prompt}
			bind:description={question.content.description}
			bind:multiple={question.content.multiple}
			bind:choices={question.content.choices}
			{editmode}
			on:change
		/>
	{/if}
</div>

<style lang="scss">
	@import './ui/variables';

	.question-container {
		display: flex;
		justify-content: center;
		align-items: center;
		margin: auto;
		padding-top: $large-padding;
	}
</style>
