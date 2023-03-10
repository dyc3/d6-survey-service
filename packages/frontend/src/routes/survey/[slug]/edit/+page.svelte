<script lang="ts">
	import { editSurvey } from '$lib/api';
	import type { Question, SurveyPatch, SurveyQuestions } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';

	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import type { PageData } from './$types';

	import _ from 'lodash';
	import { goto } from '$app/navigation';

	let title = 'Untitled Survey';
	let description = '';
	let questions: SurveyQuestions = [];

	export let data: PageData;
	title = data.survey.title;
	description = data.survey.description;
	questions = data.survey.questions;

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
		onChange('questions');
	}

	function removeQuestion(uuid: string) {
		questions = questions.filter((q) => q.uuid !== uuid);
		onChange('questions');
	}

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

	{#each questions as q}
		<Button kind="danger" size="small" on:click={() => removeQuestion(q.uuid)}>x</Button>
		<QContainer question={q.question} editmode={true} on:change={() => onChange('questions')} />
	{/each}

	<div class="panel">
		<select bind:value={questionToAdd}>
			<option value="Text">Text</option>
			<option value="MultipleChoice">Multiple Choice</option>
			<option value="Rating">Rating</option>
		</select>
		<Button size="small" on:click={() => addQuestion(questionToAdd)}>+ Add Question</Button>
	</div>

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
