<script lang="ts">
	import type { Question } from './common';
	import QMultipleChoice from './questions/QMultipleChoice.svelte';
	import QRating from './questions/QRating.svelte';
	import QTextInput from './questions/QTextInput.svelte';

	export let editmode = false;
	export let question: Question;
</script>

<div class='question-container'>
	{#if question.type === 'Text'}
		<QTextInput {...question.content} {editmode} />
	{:else if question.type === 'Rating'}
		<QRating {...question.content} {editmode} />
	{:else if question.type == 'MultipleChoice'}
		<QMultipleChoice {...question.content} {editmode} />
	{/if}
</div>

<style lang="scss">
	@import './ui/variables';

	.question-container{
		display: flex;
		justify-content: center;
		align-items: center;
		margin:auto;
		padding-top: $large-padding;
	}
</style>