<script lang="ts">
	import type { Choice } from '$lib/common';
	import Button from '$lib/ui/Button.svelte';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import Container from '$lib/ui/Container.svelte';

	export let editmode = false;
	export let prompt: string;
	export let description: string;
	export let choices: Choice[] = [];

	function addChoice() {
		choices = [...choices, { uuid: crypto.randomUUID(), text: '' }];
	}
	function removeChoice(index: number) {
		choices = choices.filter((_, i) => i !== index);
	}

	let group_selected: number | undefined = undefined;
</script>

<Container>
	<div>
		{#if editmode}
			<TextBox placeholder="Enter prompt..." bind:value={prompt} />
		{:else}
			<span class="prompt-text">{prompt}</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox placeholder="Enter description..." bind:value={description} />
		{:else}
			<span class="description-text">{description}</span>
		{/if}
	</div>

	<div class="choices">
		{#if editmode}
			{#each choices as choice, i}
				<div class="editable-choice">
					<TextBox bind:value={choice.text} placeholder="Enter text..." />
					<Button kind="danger" size="small" on:click={() => removeChoice(i)}>x</Button>
				</div>
			{/each}
			<Button on:click={addChoice}>+</Button>
		{:else}
			<ButtonGroup orientation="vertical" buttons={choices.map((choice) => choice.text)} forceSelection = {false} bind:selected={group_selected} />
		{/if}

	</div>
</Container>

<style lang="scss">
	@import '../ui/variables';

	.choices {
		display: flex;
		flex-direction: column;
	}

	.editable-choice {
		display: flex;
		flex-direction: row;
	}
	
	.prompt-text{
		font-size: 1.5em;
		font-weight: bold;
		color: $color-blue;
	}

	.description-text{
		font-size: 1em;
		color: $color-blue;
	}
</style>
