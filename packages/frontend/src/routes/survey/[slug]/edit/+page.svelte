<script lang="ts">
	import { editSurvey, isValidationError } from '$lib/api';
	import type { SurveyPatch, SurveyQuestions, ValidationError } from '$lib/common';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import type { PageData } from './$types';

	import _ from 'lodash';
	import { goto } from '$app/navigation';
	import QuestionsEditor from '$lib/QuestionsEditor.svelte';
	import ValidationErrorRenderer from '$lib/ValidationErrorRenderer.svelte';

	let title = 'Untitled Survey';
	let description = '';
	let questions: SurveyQuestions = [];

	let isSaving = false;
	let validationErrors: Map<string, ValidationError[]> = new Map();

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
			if (isValidationError(resp.error)) {
				applyValidationErrors(resp.error.message.ValidationError);
			} else {
				alert(`Error saving survey: ${resp.error.message}`);
			}
		}
		isSaving = false;
	}

	let submitChangesDebounced = _.debounce(submitChanges, 2000);

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
		isSaving = true;
		dirtyFields.add(field);
		submitChangesDebounced();
	}

	function applyValidationErrors(errors: ValidationError[]) {
		let newerrors: typeof validationErrors = new Map();
		errors.forEach((err) => {
			let prev: ValidationError[] | undefined;
			switch (err.type) {
				case 'BadValue':
					prev = newerrors.get(err.data.field);
					if (prev) {
						prev.push(err);
					} else {
						newerrors.set(err.data.field, [err]);
					}
					break;
				case 'Required':
					prev = newerrors.get(err.data.field);
					if (prev) {
						prev.push(err);
					} else {
						newerrors.set(err.data.field, [err]);
					}
					break;
				case 'Inner':
					prev = newerrors.get(err.data.field);
					if (prev) {
						prev.push(err);
					} else {
						newerrors.set(err.data.field, [err]);
					}
					break;
				default:
					console.warn('Unknown validation error type', err);
					break;
			}
		});
		validationErrors = newerrors;
	}
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
		{#each validationErrors.get('title') ?? [] as err}
			<ValidationErrorRenderer error={err} />
		{/each}
		<TextBox
			placeholder="Survey Description"
			bind:value={description}
			on:change={() => onChange('description')}
		/>
		{#each validationErrors.get('description') ?? [] as err}
			<ValidationErrorRenderer error={err} />
		{/each}
	</div>

	<QuestionsEditor bind:questions on:change={() => onChange('questions')} />

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
