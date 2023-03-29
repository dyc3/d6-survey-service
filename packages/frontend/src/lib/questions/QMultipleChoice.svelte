<script lang="ts">
	import type { Choice, Response, ValidationError } from '$lib/common';
	import Button from '$lib/ui/Button.svelte';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import Container from '$lib/ui/Container.svelte';
	import { createEventDispatcher } from 'svelte';
	import './questions.scss';
	import ValidationErrorRenderer from '$lib/ValidationErrorRenderer.svelte';
	import { buildErrorMapFromFields } from '$lib/validation';

	export let editmode = false;
	export let prompt: string;
	export let description: string;
	export let multiple = false;
	export let choices: Choice[] = [];
	export let required = false;

	let dispatch = createEventDispatcher();

	function addChoice() {
		choices = [...choices, { uuid: crypto.randomUUID(), text: '' }];
		dispatch('change');
	}
	function removeChoice(index: number) {
		choices = choices.filter((_, i) => i !== index);
		dispatch('change');
	}

	let group_selected: number | undefined = undefined;

	export let response: Response | undefined = undefined;
	$: {
		if (response !== undefined && group_selected === undefined) {
			if (response.type === 'MultipleChoice') {
				// TODO: handle multiple selections
				group_selected = choices.findIndex((choice) => {
					if (response !== undefined && response.type === 'MultipleChoice') {
						return response.content.selected.includes(choice.uuid);
					}
				});
			}
		}
		if (group_selected !== undefined) {
			// TODO: handle multiple selections
			response = { type: 'MultipleChoice', content: { selected: [choices[group_selected].uuid] } };
		} else {
			response = undefined;
		}
	}

	export let errors: ValidationError[] = [];
	$: validationErrors = buildErrorMapFromFields(errors);
</script>

<Container>
	{#if required}
		<span class="required">*</span>
	{/if}

	<div class="prompt-text">
		{#if editmode}
			<TextBox placeholder="Enter prompt..." bind:value={prompt} on:change />
			{#each validationErrors.get('prompt') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div class="description-text">
		{#if editmode}
			<TextBox placeholder="Enter description..." bind:value={description} on:change />
			{#each validationErrors.get('description') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<div class="choices">
		{#if editmode}
			{#each choices as choice, i}
				<div class="editable-choice">
					<TextBox bind:value={choice.text} placeholder="Enter text..." on:change />
					<Button kind="danger" size="small" on:click={() => removeChoice(i)}>x</Button>
				</div>
			{/each}
			<Button on:click={addChoice}>+</Button>
			<!-- TODO: show errors under the choices that have them -->
			{#each validationErrors.get('choices') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{:else}
			<ButtonGroup
				orientation="vertical"
				buttons={choices.map((choice) => choice.text)}
				forceSelection={false}
				bind:selected={group_selected}
			/>
			{#each validationErrors.get('response') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
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

	.prompt-text {
		font-size: $bold-font-size;
		font-weight: bold;
		color: $color-blue;
	}

	.description-text {
		font-size: $main-font-size;
		color: $color-blue;
	}
</style>
