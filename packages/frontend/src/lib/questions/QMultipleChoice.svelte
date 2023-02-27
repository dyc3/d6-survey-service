<script lang="ts">
	import type { Choice } from '$lib/common';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';

	export let editmode = false;
	export let prompt: string;
	export let description: string;
	export let choices: Choice[] = [];

	function addChoice() {
		choices = [...choices, { uuid: crypto.randomUUID(), text: '' }];
	}
</script>

<div>
	<div>
		{#if editmode}
			<TextBox bind:value={prompt} />
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={description} />
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<div class="choices">
		<!-- TODO: make this a button group, see #75 -->
		{#each choices as choice}
			{#if editmode}
				<TextBox bind:value={choice.text} placeholder="Enter text..." />
			{:else}
				<Button>{choice.text}</Button>
			{/if}
		{/each}
		{#if editmode}
			<Button on:click={addChoice}>+</Button>
		{/if}
	</div>
</div>

<style lang="scss">
	.choices {
		display: flex;
		flex-direction: column;
	}
</style>
