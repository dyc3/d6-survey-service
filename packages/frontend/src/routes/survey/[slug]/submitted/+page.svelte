<script lang="ts">
	import type { PageData } from './$types';
	import TextBox from '$lib/ui/TextBox.svelte';

	export let data: PageData;

	let responseUuid = new URLSearchParams(window.location.search).get('response');

	$: responseUrl = `${window.location.origin}/survey/${data.slug}/respond?response=${responseUuid}`;
</script>

<h1>Response Submitted</h1>

<div>
	<span>Save this link to edit your response later:</span>
</div>

<div>
	<TextBox value={responseUrl} />
</div>

<div>
	<a href={responseUrl}>Or click here to edit it right away.</a>
</div>
