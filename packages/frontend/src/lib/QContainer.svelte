<script lang="ts">
	import type { Question } from './common';
	import QTextInput from './questions/QTextInput.svelte';

	export let editmode = false;
	export let question: Question;
</script>

{#if question.type === 'Text'}
	<QTextInput {...question.content} {editmode} />
{/if}
