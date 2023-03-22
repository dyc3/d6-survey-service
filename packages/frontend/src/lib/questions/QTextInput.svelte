<script lang="ts">
	import TextBox from '$lib/ui/TextBox.svelte';
	import Container from '$lib/ui/Container.svelte';

	export let editmode = false;
	export let multiline = false;
	let response = '';
	export let prompt: string;
	export let description: string;
	export let required = false;
</script>

<Container>
	{#if required}
		<span class="required">*</span>
	{/if}

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

	<TextBox bind:value={response} disabled={editmode} {multiline} />
</Container>

<style lang="scss">
	@import '../ui/variables';
	
	.required{
		color: red;
		position: absolute;
		right: 2.5%;
		top: 2.5%;
	}
</style>