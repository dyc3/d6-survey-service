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
	import { buildErrorMapFromFields } from '$lib/validation';

	let title = 'Untitled Survey';
	let description = '';
	let questions: SurveyQuestions = [];

	let isSaving = false;
	let wasSaveSuccessful = true;
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

		try {
			let resp = await editSurvey(data.surveyId, _.pick(patch, Array.from(dirtyFields)));
			if (resp.ok) {
				wasSaveSuccessful = true;
				dirtyFields.clear();
			} else {
				wasSaveSuccessful = false;
				if (isValidationError(resp.error)) {
					applyValidationErrors(resp.error.message.ValidationError);
				} else {
					alert(`Error saving survey: ${resp.error.message}`);
				}
			}
		} catch (e) {
			console.error(e);
			wasSaveSuccessful = false;
		} finally {
			isSaving = false;
		}
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
			alert('Error publishing survey: ' + resp.error.message);
		}
	}

	function onChange(field: keyof SurveyPatch) {
		isSaving = true;
		dirtyFields.add(field);
		submitChangesDebounced();
	}

	function applyValidationErrors(errors: ValidationError[]) {
		validationErrors = buildErrorMapFromFields(errors);
	}

	$: {
		title = title;
		onChange('title');
	}

	$: {
		description = description;
		onChange('description');
	}
</script>

<div class="toolbar">
	<div>
		<h1>{title}</h1>
		<h2>Editing</h2>
		{#if isSaving}
			<span>Saving...</span>
		{:else if wasSaveSuccessful}
			<span>Changes saved</span>
		{:else}
			<span>Changes not saved</span>
		{/if}
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

	<QuestionsEditor
		bind:questions
		on:change={() => onChange('questions')}
		errors={validationErrors.get('questions') ?? []}
	/>

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
