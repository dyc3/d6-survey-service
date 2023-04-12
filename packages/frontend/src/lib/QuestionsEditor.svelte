<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { Question, SurveyQuestions, ValidationError } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import { buildErrorMapFromUuids } from '$lib/validation';
	import Draggable from './Draggable.svelte';

	export let questions: SurveyQuestions = [];
	export let errors: ValidationError[] = [];

	let questionToAdd: 'Text' | 'Rating' | 'MultipleChoice' = 'Text';
	const dispatch = createEventDispatcher();

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
	<div class='dragAndDropArea'>
		<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
			<path d="M20 9H4V11H20V9ZM4 15H11V13H4V15ZM20 13V15H13V13H20Z" fill="currentColor"/>
		</svg>
	</div>
	<Draggable>
		<QContainer
			bind:question={q.question}
			bind:required={q.required}
			editmode={true}
			on:change
			errors={errorsByUUID.get(q.uuid) ?? []}
		/>
	</Draggable>
{/each}

<div class="panel">
	<select bind:value={questionToAdd}>
		<option value="Text">Text</option>
		<option value="MultipleChoice">Multiple Choice</option>
		<option value="Rating">Rating</option>
	</select>
	<Button --margin="5px" size="small" on:click={() => addQuestion(questionToAdd)}>+ Add Question</Button>
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
