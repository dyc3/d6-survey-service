<script lang="ts">
	import type { Survey } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	let survey: Survey = data.survey;
</script>

<h1>{survey.title}</h1>
<p>{survey.description}</p>

{#each survey.questions as surveyquestion}
	<QContainer question={surveyquestion.question} />
{/each}

<Button size="large" kind="primary">Submit</Button>
