<script lang="ts">
	import { editSurvey, getSurveyAuth } from '$lib/api';
	import type { Question, SurveyPatch, SurveyQuestions } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';

	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import type { PageData } from './$types';

	import _ from 'lodash';
	import { onMount } from 'svelte';
	import { error } from '@sveltejs/kit';

	let title = 'Untitled Survey';
	let description = '';
	let questions: SurveyQuestions = [];

	export let data: PageData;

	function buildQuestion(type: 'Text' | 'Rating' | 'MultipleChoice'): Question {
		let question: Question;
		switch (type) {
			case 'Text':
				question = {
					type: 'Text',
					content: {
						prompt: '',
						description: '',
						multiline: false
					}
				};
				break;
			case 'Rating':
				question = {
					type: 'Rating',
					content: {
						prompt: '',
						description: '',
						max_rating: 5
					}
				};
				break;
			case 'MultipleChoice':
				question = {
					type: 'MultipleChoice',
					content: {
						prompt: '',
						description: '',
						multiple: false,
						choices: []
					}
				};
				break;
			default:
				throw new Error('Invalid question type');
		}
		return question;
	}

	function addQuestion(type: 'Text' | 'Rating' | 'MultipleChoice') {
		questions = [
			...questions,
			{
				uuid: crypto.randomUUID(),
				required: false,
				question: buildQuestion(type)
			}
		];
		onChange();
	}

	function removeQuestion(uuid: string) {
		questions = questions.filter((q) => q.uuid !== uuid);
		onChange();
	}

	async function submitChanges() {
		let patch: SurveyPatch = {
			title,
			description,
			questions
		};

		await editSurvey(data.surveyId, patch);
	}

	let onChange = _.debounce(submitChanges, 1000);

	onMount(async () => {
		let response = await getSurveyAuth(data.surveyId);
		if (!response.ok) {
			if (response.error.message === 'NotFound') {
				throw error(404, 'Survey not found');
			} else if (response.error.message === 'NotPublished') {
				throw error(403, 'Survey not published');
			} else {
				throw error(500, 'Internal server error');
			}
		}

		let survey = response.value;
		title = survey.title;
		description = survey.description;
		questions = survey.questions;
	});

	let questionToAdd: 'Text' | 'Rating' | 'MultipleChoice' = 'Text';
</script>

<div class="toolbar">
	<div>
		<h1>Sample Survey Title</h1>
		<h2>Editing</h2>
	</div>
	<Button>View Results</Button>
</div>

<div class="container">
	<TextBox placeholder="Survey Title" bind:value={title} />
	<TextBox placeholder="Survey Description" bind:value={description} />

	{#each questions as q}
		<Button kind="danger" size="small" on:click={() => removeQuestion(q.uuid)}>x</Button>
		<QContainer question={q.question} editmode={true} on:change={onChange} />
	{/each}
	<select bind:value={questionToAdd}>
		<option value="Text">Text</option>
		<option value="MultipleChoice">Multiple Choice</option>
		<option value="Rating">Rating</option>
	</select>

	<Button on:click={() => addQuestion(questionToAdd)}>+</Button>

	<Button>Publish Survey</Button>
</div>

<style lang="scss">
	@import '../../../../lib/ui/variables.scss';

	.container {
		border: 2px solid $color-default;
		align-items: center;
	}

	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>
