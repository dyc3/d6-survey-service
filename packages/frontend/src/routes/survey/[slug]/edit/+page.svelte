<script lang="ts">
	import { editSurvey } from '$lib/api';
	import type { SurveyPatch, SurveyQuestions } from '$lib/common';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import type { PageData } from './$types';

	import _ from 'lodash';
	import { goto } from '$app/navigation';
	import QuestionsEditor from '$lib/QuestionsEditor.svelte';

	let title = 'Untitled Survey';
	let description = '';
	let questions: SurveyQuestions = [];

	export let data: PageData;
	title = data.survey.title;
	description = data.survey.description;
	questions = data.survey.questions;

	let dirtyFields: Set<keyof SurveyPatch> = new Set();

	async function submitChanges() {
		let patch: SurveyPatch = {
			title,
			description,
			questions
		};

		let resp = await editSurvey(data.surveyId, _.pick(patch, Array.from(dirtyFields)));
		if (resp.ok) {
			dirtyFields.clear();
		} else {
			alert('Error saving changes');
		}
	}

	let submitChangesDebounced = _.debounce(submitChanges, 1000);

	async function publishSurvey() {
		// make sure we don't submit changes twice, or publish a survey with unsaved changes
		submitChangesDebounced.cancel();
		let resp = await editSurvey(data.surveyId, {
			..._.pick(
				{
					title,
					description,
					questions
				},
				Array.from(dirtyFields)
			),
			published: true
		});
		if (resp.ok) {
			await goto(`/mysurveys`);
		} else {
			alert('Error publishing survey');
		}
	}

	function onChange(field: keyof SurveyPatch) {
		dirtyFields.add(field);
		submitChangesDebounced();
	}

	let questionToAdd: 'Text' | 'Rating' | 'MultipleChoice' = 'Text';
</script>

<div class="toolbar">
	<div>
		<h1>{title}</h1>
		<h2>Editing</h2>
	</div>
	<Button>View Results</Button>
</div>

<div class="container">
	<div class="panel">
		<TextBox placeholder="Survey Title" bind:value={title} on:change={() => onChange('title')} />
		<TextBox
			placeholder="Survey Description"
			bind:value={description}
			on:change={() => onChange('description')}
		/>
	</div>

	<QuestionsEditor bind:questions />

	<div class="panel">
		<Button on:click={publishSurvey}>Publish Survey</Button>
	</div>
</div>

<style lang="scss">
	@import '../../../../lib/ui/variables';

	.container {
		border: 2px solid $color-default;
		align-items: center;
	}

	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.panel {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		margin: 40px;
	}
</style>
