<script lang="ts">
	import { editSurvey, exportResponses, isValidationError } from '$lib/api';
	import type { SurveyPatch, SurveyQuestions, ValidationError } from '$lib/common';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import Spinner from '$lib/ui/Spinner.svelte';
	import type { PageData } from './$types';

	import _ from 'lodash';
	import { goto } from '$app/navigation';
	import QuestionsEditor from '$lib/QuestionsEditor.svelte';
	import ValidationErrorRenderer from '$lib/ValidationErrorRenderer.svelte';
	import { buildErrorMapFromFields } from '$lib/validation';
	import { page } from '$app/stores';
	import Panel from '$lib/ui/Panel.svelte';

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

	let loadingPublish = false;

	async function publishSurvey() {
		loadingPublish = true;
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
			loadingPublish = false;
		}
	}

	function onChange(field: keyof SurveyPatch) {
		isSaving = true;
		dirtyFields.add(field);
		validationErrors.delete(field);
		validationErrors = validationErrors;
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

	let loadingExport = false;

	async function downloadResults() {
		loadingExport = true;
		let resp = await exportResponses(data.surveyId);
		loadingExport = false;
		if (!resp.ok) {
			alert('Error exporting results: ' + resp.error.message);
			return;
		}
		let obj = URL.createObjectURL(new Blob([resp.value.blob], { type: 'text/csv' }));

		const a = document.createElement('a');
		a.style.display = 'none';
		a.href = obj;
		a.download = resp.value.filename;
		document.body.appendChild(a);
		a.click();

		URL.revokeObjectURL(obj);
	}
</script>

<div class="toolbar">
	<div>
		<h1>{title}</h1>
		<h2>Editing</h2>
		<span
			class="save-indicator"
			class:saving={isSaving}
			class:success={!isSaving && wasSaveSuccessful}
			class:fail={!isSaving && !wasSaveSuccessful}
		>
			{#if isSaving}
				<Spinner /> Saving...
			{:else if wasSaveSuccessful}
				Changes saved
			{:else}
				Changes not saved
			{/if}
		</span>
	</div>
	<Button --margin="5px" on:click={downloadResults} loading={loadingExport}>Export Results</Button>
</div>

<div class="container">
	<Panel --spacing="0">
		<TextBox
			--margin="0"
			placeholder="Survey Title"
			bind:value={title}
			on:change={() => onChange('title')}
		/>
		{#each validationErrors.get('title') ?? [] as err}
			<ValidationErrorRenderer error={err} />
		{/each}
	</Panel>
	<Panel>
		<TextBox
			--margin="0"
			placeholder="Survey Description"
			bind:value={description}
			on:change={() => onChange('description')}
		/>
		{#each validationErrors.get('description') ?? [] as err}
			<ValidationErrorRenderer error={err} />
		{/each}
	</Panel>

	<QuestionsEditor
		bind:questions
		on:change={() => onChange('questions')}
		errors={validationErrors.get('questions') ?? []}
	/>

	<Panel>
		<Button --margin="5px" on:click={publishSurvey} loading={loadingPublish}>Publish Survey</Button>
	</Panel>
</div>

<style lang="scss">
	@import '../../../../lib/ui/variables';

	.container {
		align-items: center;
	}

	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.save-indicator.fail {
		color: $color-danger;
	}
</style>
