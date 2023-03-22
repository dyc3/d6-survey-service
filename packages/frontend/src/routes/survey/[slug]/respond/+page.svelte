<script lang="ts">
	import type { Survey, SurveyResponses } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';
	import { createSurveyResponse, editSurveyResponse } from '$lib/api';

	export let data: PageData;

	let survey: Survey = data.survey;

	let response: SurveyResponses = {};

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

	async function submitResponse() {
		let query = parseQuery(window.location.search);
		let responderUuid = query.responder;
		try {
			submitInProgress = true;
			if (!responderUuid) {
				let resp = await createSurveyResponse(survey.id, response);
				if (resp.ok) {
					responderUuid = resp.value.responder_uuid;
					goto(`/survey/${survey.id}/submitted?response=${responderUuid}`);
				} else {
					alert(JSON.stringify(resp.error));
					console.error(resp.error);
				}
			} else {
				let resp = await editSurveyResponse(survey.id, responderUuid, response);
				if (resp.ok) {
					goto(`/survey/${survey.id}/submitted?response=${responderUuid}`);
				} else {
					alert(JSON.stringify(resp.error));
					console.error(resp.error);
				}
			}
		} catch (e) {
			console.error(e);
		} finally {
			submitInProgress = false;
		}
	}
</script>

<h1>{survey.title}</h1>
<p>{survey.description}</p>

{#each survey.questions as surveyquestion}
	<QContainer question={surveyquestion.question} />
{/each}

<Button size="large" kind="primary" on:click={submitResponse}>Submit</Button>
