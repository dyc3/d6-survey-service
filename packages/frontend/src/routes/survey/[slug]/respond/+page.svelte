<script lang="ts">
	import type { Survey, SurveyResponses, ValidationError } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { createSurveyResponse, editSurveyResponse, isValidationError } from '$lib/api';
	import { buildErrorMapFromUuids } from '$lib/validation';

	export let data: PageData;

	let survey: Survey = data.survey;

	let response: SurveyResponses = data.surveyResponse;

	let submitInProgress = false;
	let validationErrors: Map<string, ValidationError[]> = new Map();

	async function submitResponse() {
		let query = new URLSearchParams(window.location.search);
		let responderUuid = query.get('responder');
		try {
			submitInProgress = true;
			let resp = await (responderUuid
				? editSurveyResponse(survey.id, responderUuid, response)
				: createSurveyResponse(survey.id, response));
			if (resp.ok) {
				if (resp.value !== null) {
					responderUuid = resp.value.responder_uuid;
				}
				//override the event listener then properly remove it below.
				window.onbeforeunload = function () {
					return;
				}

				window.removeEventListener('beforeunload', window.onbeforeunload);
				goto(`/survey/${survey.id}/submitted?responder=${responderUuid}`);
			} else {
				if (isValidationError(resp.error)) {
					applyValidationErrors(resp.error.message.ValidationError);
				} else {
					// TODO: don't alert, show this on the page instead.
					alert(`Error saving survey: ${resp.error.message}`);
				}
			}
		} catch (e) {
			console.error(e);
		} finally {
			submitInProgress = false;
		}
	}

	//alert user if leaving response page
	if (browser) {
		window.onbeforeunload = function () {
			return 'Are you sure you want to leave this page? Your response will not be saved.';
		};
	}
	
	function applyValidationErrors(errors: ValidationError[]) {
		validationErrors = buildErrorMapFromUuids(errors);
	}
</script>

<h1>{survey.title}</h1>
<p>{survey.description}</p>

{#each survey.questions as surveyquestion}
	<QContainer
		question={surveyquestion.question}
		bind:response={response[surveyquestion.uuid]}
		required={surveyquestion.required}
		errors={validationErrors.get(surveyquestion.uuid) ?? []}
	/>
{/each}

{#if submitInProgress}
	<div>Submmitting...</div>
{/if}

<div class='submit-button'>
	<Button --margin="5px" size="large" kind="primary" on:click={submitResponse}>Submit</Button>
</div>


<style lang="scss">
	@import '../../../../lib/ui/variables';
	
	.submit-button{
		display: flex;
		justify-content: center;
		margin-top: $large-padding
	}

</style>