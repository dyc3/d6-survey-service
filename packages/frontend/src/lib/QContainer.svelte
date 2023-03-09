<script lang="ts">
	import type { Question } from './common';
	import QMultipleChoice from './questions/QMultipleChoice.svelte';
	import QRating from './questions/QRating.svelte';
	import QTextInput from './questions/QTextInput.svelte';

	export let editmode = false;
	export let question: Question;
</script>

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
		{editmode}
		on:change
	/>
{/if}
