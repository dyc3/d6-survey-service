<script lang="ts">
	import { slide } from 'svelte/transition';
	import { flip } from 'svelte/animate';

	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import type { ListedSurvey } from '$lib/common';
	import { createSurvey, deleteSurvey } from '$lib/api';
	import { goto } from '$app/navigation';
	import type { PageData } from './$types';

	export let data: PageData;
	let surveys: ListedSurvey[] = data.surveys;

	let loadingCreate = false;
	async function createNewSurvey() {
		loadingCreate = true;
		let surveyInfo = await createSurvey();
		if (surveyInfo.ok) {
			goto('/survey/' + surveyInfo.value.id + '/edit');
		} else {
			console.error(surveyInfo.error);
		}
	}

	async function doDeleteSurvey(survey_id: number) {
		let confirm = window.confirm('Are you sure you want to delete this survey?');
		if (!confirm) {
			return;
		}
		let resp = await deleteSurvey(survey_id);
		if (resp.ok) {
			surveys = surveys.filter((survey) => survey.id !== survey_id);
		} else {
			console.error(resp.error);
		}
	}
</script>

<div class="toolbar">
	<h1>My Surveys</h1>
	<Button kind="primary" size="large" on:click={createNewSurvey} loading={loadingCreate}
		>Create Survey</Button
	>
</div>
<div class="main-container">
	<table class="container">
		<thead class="header">
			<th class="name">Name</th>
			<th class="published">Published</th>
			<th class="share-link">Share Link</th>
			<th class="actions">Actions</th>
		</thead>
		<tbody>
			{#each surveys as survey (survey.id)}
				<tr class="survey" transition:slide|local animate:flip>
					<td class="name">{survey.title}</td>
					<!-- TODO: replace with check box-->
					<td class="published">{survey.published ? 'Yes' : 'No'}</td>

					<td class="share-link">
						<TextBox
							value="{window.location.origin}/survey/{survey.id}/respond"
							disabled
							copyable
						/>
					</td>
					<td class="actions">
						<Button --margin="5px" size="small" on:click={() => goto(`/survey/${survey.id}/edit`)}>
							Edit
						</Button>
						<Button
							--margin="5px"
							size="small"
							kind="danger"
							on:click={() => doDeleteSurvey(survey.id)}
						>
							Delete
						</Button>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style lang="scss">
	@import '../../lib/ui/variables';

	.main-container {
		border: 2px solid $color-default;
		min-width: fit-content;
	}

	.container {
		width: 100%;
		border-collapse: collapse;
	}

	.header {
		align-items: center;
		text-align: left;
		font-weight: 500;
		border-bottom: 2px solid $color-default;
	}

	.actions {
		display: flex;
 		flex-direction: row;
 		align-items: center;
	}

	.survey {
		align-items: center;
		border: 1px solid #426881;
		max-height: 50px;
		margin-bottom: 20px;
	}

	.published {
		text-align: center;
	}

	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	h1 {
		font-weight: 700;
	}

	td {
		font-weight: 400;
		color: $color-default;
	}
</style>
