<script lang="ts">
	import { slide } from 'svelte/transition';

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

	$: items = choices.map((choice) => {
		return { label: choice.text, value: choice.uuid };
	});

	export let response: Response | undefined = undefined;
	let selected: string[] = loadResponse(response);

	function loadResponse(response: Response | undefined): string[] {
		if (response !== undefined && response.type === 'MultipleChoice') {
			return response.content.selected;
		}
		return [];
	}

	function setResponse(uuids: string[]) {
		if (uuids.length > 0) {
			response = { type: 'MultipleChoice', content: { selected: uuids } };
		} else {
			response = undefined;
		}
	}
	$: setResponse(selected);

	export let errors: ValidationError[] = [];
	$: validationErrors = buildErrorMapFromFields(errors);
</script>

<div class="question">
	{#if required}
		<span class="required">*</span>
	{/if}

	<div class="prompt-text">
		{#if editmode}
			<TextBox placeholder="Enter prompt..." bind:value={prompt} on:change />
			<div>
				{#each validationErrors.get('prompt') ?? [] as error}
					<ValidationErrorRenderer {error} />
				{/each}
			</div>
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div class="description-text">
		{#if editmode}
			<TextBox placeholder="Enter description..." bind:value={description} on:change />
			<div>
				{#each validationErrors.get('description') ?? [] as error}
					<ValidationErrorRenderer {error} />
				{/each}
			</div>
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<div class="instructions">
		{#if multiple}
			Select all that apply.
		{:else}
			Select one.
		{/if}
	</div>

	<div class="choices">
		{#if editmode}
			<div>
				<label for="multiple">Multiple</label>
				<input type="checkbox" id="multiple" bind:checked={multiple} on:change />
			</div>
			{#each choices as choice, i (choice.uuid)}
				<div class="editable-choice" transition:slide|local={{ duration: 200 }}>
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
				buttons={items}
				forceSelection={false}
				{multiple}
				bind:selected
			/>
			{#each validationErrors.get('response') ?? [] as error}
				<ValidationErrorRenderer {error} />
			{/each}
		{/if}
	</div>
</div>

<style lang="scss">
	@import '../ui/variables';

	.choices {
		display: flex;
		flex-direction: column;
		min-width: 300px;
	}

	.editable-choice {
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.instructions {
		font-style: italic;
		opacity: 0.6;
	}
</style>
