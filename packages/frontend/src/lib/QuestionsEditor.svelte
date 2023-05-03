<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { slide } from 'svelte/transition';
	import type { Question, SurveyQuestions, ValidationError } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import { buildErrorMapFromUuids } from '$lib/validation';
	import Draggable from './Draggable.svelte';
	import { arrayMove } from './arrayutils';
	import Panel from './ui/Panel.svelte';

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
						max_rating: 5,
						min_text: '',
						max_text: ''
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

	//listen for move event
	function handleMove(e: CustomEvent) {
		const { oldIndex, newIndex } = e.detail;
		if (oldIndex === newIndex) return;
		questions = arrayMove(questions, oldIndex, newIndex);
		dispatch('change');
	}
</script>

{#each questions as q, index (q.uuid)}
	<Panel border={true}>
		<div transition:slide|local={{ duration: 600 }}>
			<Button kind="danger" size="small" on:click={() => removeQuestion(q.uuid)}>X</Button>
			<Draggable {index} on:move={handleMove}>
				<QContainer
					bind:question={q.question}
					bind:required={q.required}
					editmode={true}
					on:change
					errors={errorsByUUID.get(q.uuid) ?? []}
				/>
			</Draggable>
		</div>
	</Panel>
{/each}

<Panel>
	<div class="flex-center" style="flex-direction: column">
		<select bind:value={questionToAdd}>
			<option value="Text">Text</option>
			<option value="MultipleChoice">Multiple Choice</option>
			<option value="Rating">Rating</option>
		</select>
		<Button --margin="5px" size="small" on:click={() => addQuestion(questionToAdd)}>
			+ Add Question
		</Button>
	</div>
</Panel>
