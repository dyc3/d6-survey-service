<script lang="ts">
	import type { Response } from '$lib/common';
	import TextBox from '$lib/ui/TextBox.svelte';

	export let editmode = false;
	export let multiline = false;
	export let prompt: string;
	export let description: string;

	let responseContent = '';
	export let response: Response | undefined = undefined;

	$: response = { type: 'Text', content: { text: responseContent } };
</script>

<div>
	<div>
		{#if editmode}
			<TextBox bind:value={prompt} placeholder="Prompt" on:change />
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={description} placeholder="Description" on:change />
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<TextBox bind:value={responseContent} disabled={editmode} {multiline} />
</div>
