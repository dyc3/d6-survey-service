<script lang="ts">
	import type { Survey, SurveyResponses, ValidationError } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';
	import { createSurveyResponse, editSurveyResponse, isValidationError } from '$lib/api';
	import { buildErrorMapFromUuids } from '$lib/validation';

	export let data: PageData;

	let survey: Survey = data.survey;

	let response: SurveyResponses = data.surveyResponse;

	// TODO: replace this with just URLSearchParams?
	function parseQuery(queryString: string): { [key: string]: string } {
		let query: { [key: string]: string } = {};
		let pairs = (queryString[0] === '?' ? queryString.substr(1) : queryString).split('&');
		for (var i = 0; i < pairs.length; i++) {
			var pair = pairs[i].split('=');
			query[decodeURIComponent(pair[0])] = decodeURIComponent(pair[1] || '');
		}
		return query;
	}

	let submitInProgress = false;
	let validationErrors: Map<string, ValidationError[]> = new Map();

	async function submitResponse() {
		let query = parseQuery(window.location.search);
		let responderUuid = query.responder;
		try {
			submitInProgress = true;
			let resp = await (responderUuid
				? editSurveyResponse(survey.id, responderUuid, response)
				: createSurveyResponse(survey.id, response));
			if (resp.ok) {
				if (resp.value !== null) {
					responderUuid = resp.value.responder_uuid;
				}
				goto(`/survey/${survey.id}/submitted?response=${responderUuid}`);
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

<Button size="large" kind="primary" on:click={submitResponse}>Submit</Button>
