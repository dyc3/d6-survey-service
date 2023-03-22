<script lang="ts">
	import Button from '../../lib/ui/Button.svelte';
	import TextBox from '../../lib/ui/TextBox.svelte';
	import type { ListedSurvey } from '../../lib/common';
	import { createSurvey } from '$lib/api';
	import { goto } from '$app/navigation';

	async function createNewSurvey() {
		let surveyInfo = await createSurvey();
		if (surveyInfo.ok) {
			goto('/survey/' + surveyInfo.value.id + '/edit');
		}
	}
	
	let surveys: ListedSurvey[] = [];
</script>

<div class="toolbar">
	<h1>My Surveys</h1>
	<Button kind="primary" size="large" on:click={ createNewSurvey }>Create Survey</Button>
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
			{#each surveys as survey}
				<tr class="survey">
					<td class="name">{survey.title}</td>
					<!-- TODO: replace with check box-->
					<td class="published">{survey.published ? 'Yes' : 'No'}</td>

					<!-- TODO: make this read only-->
					<td class="share-link">
						<TextBox value="{window.location.origin}/survey/{survey.id}/respond" />
					</td>
					<td class="actions">
						<Button>Edit</Button>
						<Button kind="danger">Delete</Button>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style lang="scss">
	@import '../../lib/ui/variables';

	.main-container {
		height: 70vh;
		border: 2px solid $color-default;
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
		max-width: 250px;
		justify-content: space-between;
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
