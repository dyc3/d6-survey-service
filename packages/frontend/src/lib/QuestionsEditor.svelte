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

	let draggedIndex: number | null = null;
	let hoveringIndex: number | null = null;
	let sidebarVisible = false;

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
		sidebarVisible = false;
		dispatch('change');
	}

	function handleDragStart(e: CustomEvent) {
		draggedIndex = e.detail.start;
		sidebarVisible = true;
	}

	//handle move but using the new index from the drop container
	function handleMoveDrop(e: DragEvent) {
		if (draggedIndex === null) return;
		const oldIndex = draggedIndex;
		const newIndex = Array.from(document.querySelectorAll('.mini-drop-container')).indexOf(
			e.target as HTMLElement
		);
		questions = arrayMove(questions, oldIndex, newIndex);
		draggedIndex = null;
		hoveringIndex = null;
		sidebarVisible = false;
		dispatch('change');
	}
</script>

{#each questions as q, index (q.uuid)}
	<Panel border={true}>
		<div transition:slide|local={{ duration: 600 }}>
			<Button kind="danger" size="small" on:click={() => removeQuestion(q.uuid)}>X</Button>
			<Draggable
				{index}
				on:move={handleMove}
				on:dragbegin={handleDragStart}
				on:dragStop={() => (sidebarVisible = false)}
			>
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

<div class="drop-container {sidebarVisible ? 'show' : ''}" id="droppableContainer">
	{#each questions as q, index (q.question.content.prompt)}
		<div
			class="mini-drop-container {hoveringIndex === index ? 'hover' : ''}"
			on:drop={handleMoveDrop}
			on:dragover={(e) => e.preventDefault()}
			on:dragenter={() => {
				hoveringIndex = index;
			}}
			on:dragleave={() => {
				//make sure leaving does not fire after entering
				if (hoveringIndex === index) {
					hoveringIndex = null;
				}
			}}
		>
			{q.question.content.prompt}
		</div>
	{/each}
</div>

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

<style lang="scss">
	@import './ui/variables';

	.drop-container {
		display: flex;
		position: fixed;
		right: 0;
		opacity: 0;
		top: 50%;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background-color: white;
		border: 1px solid $color-blue;
		border-top-left-radius: 8px;
		border-bottom-left-radius: 8px;
		transition: opacity 0.5s ease-in-out;
		pointer-events: none;
	}

	.drop-container.show {
		opacity: 1;
	}

	.mini-drop-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		border-bottom: 1px solid $color-blue;
		width: 100%;
		height: 50px;
		pointer-events: all;
	}

	.mini-drop-container.hover {
		background-color: #3273dc22;
	}

	.mini-drop-container:last-of-type {
		border-bottom: none;
	}
</style>
