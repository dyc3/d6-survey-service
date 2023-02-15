<script lang="ts">
	import Button from '../../lib/ui/Button.svelte';
	import TextBox from '../../lib/ui/TextBox.svelte';
</script>

<h1>My Surveys</h1>

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
			<td class="published">Yes</td>
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

<style lang="scss">
	.container {
		width: 100%;
	}
	.header {
		align-items: center;
		text-align: left;
	}
	.survey {
		align-items: center;
	}
	.published {
		text-align: center;
	}
</style>
