<script lang="ts">
	import type { Survey } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	export let data: PageData;

	let survey: Survey = data.survey;
</script>

<h1>{survey.title}</h1>
<p>{survey.description}</p>

{#each survey.questions as surveyquestion}
	<QContainer question={surveyquestion.question} />
{/each}

<!-- TODO: actually submit survey to get the response UUID -->
<Button
	size="large"
	kind="primary"
	on:click={() =>
		goto(`/survey/${data.slug}/submitted?response=1d3fd956-2c6b-4439-9c8f-c6933f6d895c`)}
>
	Submit
</Button>
