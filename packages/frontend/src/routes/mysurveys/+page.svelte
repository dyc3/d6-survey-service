<script lang="ts">
	import Button from '../../lib/ui/Button.svelte';
	import TextBox from '../../lib/ui/TextBox.svelte';
</script>

<div class="toolbar">
	<h1>My Surveys</h1>
	<Button kind="primary" size="large">Create Survey</Button>
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
			<tr class="survey">
				<td class="name">Survey 1</td>
				<!-- TODO: replace with check box-->
				<td class="published">Yes</td>
				<!-- TODO: make this read only-->
				<td class="share-link"><TextBox value="https://www.survey.com/123" /></td>
				<td class="actions">
					<Button>Edit</Button>
					<Button>Delete</Button>
				</td>
			</tr>
			<tr class="survey">
				<td class="name">Survey 2</td>
				<td class="published">Yes</td>
				<td class="share-link"><TextBox value="https://www.survey.com/123" /></td>
				<td class="actions">
					<Button>Edit</Button>
					<Button>Delete</Button>
				</td>
			</tr>
		</tbody>
	</table>
</div>

<style lang="scss">
	.main-container {
		height: 70vh;
		border: 2px solid #426881;
	}
	.container {
		width: 100%;
		border-collapse: collapse;
	}
	.header {
		align-items: center;
		text-align: left;
		font-family: 'DM Sans', sans-serif;
		font-weight: 500;
		color: #426881;
		border-bottom: 2px solid #426881;
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
		font-family: 'DM Sans', sans-serif;
		font-weight: 700;
		color: #426881;
	}

	td {
		font-family: 'DM Sans', sans-serif;
		font-weight: 400;
		color: #426881;
	}
</style>
