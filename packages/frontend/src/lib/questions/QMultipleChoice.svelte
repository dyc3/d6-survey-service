<script lang="ts">
	import type { Choice } from '$lib/common';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';

	export let editmode = false;
	export let prompt: string;
	export let description: string;
	export let multiple = false;
	export let choices: Choice[] = [];

	function addChoice() {
		choices = [...choices, { uuid: crypto.randomUUID(), text: '' }];
		dispatchEvent(new Event('change'));
	}
	function removeChoice(index: number) {
		choices = choices.filter((_, i) => i !== index);
		dispatchEvent(new Event('change'));
	}
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

	<div class="choices">
		<!-- TODO: make this a button group, see #75 -->
		{#each choices as choice, i}
			{#if editmode}
				<div class="editable-choice">
					<TextBox bind:value={choice.text} placeholder="Enter text..." on:change />
					<Button kind="danger" size="small" on:click={() => removeChoice(i)}>x</Button>
				</div>
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

	.editable-choice {
		display: flex;
		flex-direction: row;
	}
</style>
