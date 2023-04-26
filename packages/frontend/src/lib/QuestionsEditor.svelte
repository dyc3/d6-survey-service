<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { slide } from 'svelte/transition';
	import type { Question, SurveyQuestions, ValidationError } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import { buildErrorMapFromUuids } from '$lib/validation';

	export let questions: SurveyQuestions = [];
	export let errors: ValidationError[] = [];

	let questionToAdd: 'Text' | 'Rating' | 'MultipleChoice' = 'Text';
	const dispatch = createEventDispatcher();
	let indexQ: HTMLDivElement;

	function move(oldIndex: number, newIndex: number) {
		let temp = questions[oldIndex];
		questions[oldIndex] = questions[newIndex];
		questions[newIndex] = temp;
	}

	function buildQuestion(type: 'Text' | 'Rating' | 'MultipleChoice'): Question {
		let question: Question;
		switch (type) {
			case 'Text':
				question = {
					type: 'Text',
					content: {
						prompt: '',
						description: '',
						multiline: false
					}
				};
				break;
			case 'Rating':
				question = {
					type: 'Rating',
					content: {
						prompt: '',
						description: '',
						max_rating: 5
					}
				};
				break;
			case 'MultipleChoice':
				question = {
					type: 'MultipleChoice',
					content: {
						prompt: '',
						description: '',
						multiple: false,
						choices: []
					}
				};
				break;
			default:
				throw new Error('Invalid question type');
		}
		return question;
	}

	function addQuestion(type: 'Text' | 'Rating' | 'MultipleChoice') {
		questions = [
			...questions,
			{
				uuid: crypto.randomUUID(),
				required: false,
				question: buildQuestion(type)
			}
		];
		dispatch('change');
	}

	function removeQuestion(uuid: string) {
		let confirm = window.confirm('Are you sure you want to delete this question?');
		if (!confirm) return;
		questions = questions.filter((q) => q.uuid !== uuid);
		dispatch('change');
	}

	let errorsByUUID: Map<string, ValidationError[]> = new Map();

	$: {
		errorsByUUID = buildErrorMapFromUuids(errors);
	}
</script>

{#each questions as q}
	<Button kind="danger" size="small" on:click={() => removeQuestion(q.uuid)}>X</Button>
	<QContainer
		bind:question={q.question}
		bind:required={q.required}
		editmode={true}
		on:change
		errors={errorsByUUID.get(q.uuid) ?? []}
	/>
{/each}

<div class="panel">
	<select bind:value={questionToAdd}>
		<option value="Text">Text</option>
		<option value="MultipleChoice">Multiple Choice</option>
		<option value="Rating">Rating</option>
	</select>
	<Button --margin="5px" size="small" on:click={() => addQuestion(questionToAdd)}>
		+ Add Question
	</Button>
</div>

<style lang="scss">
	// TODO: deduplicate this class, copied from survey edit page
	.panel {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		margin: 40px;
	}
</style>
