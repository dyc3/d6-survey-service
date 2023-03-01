<script lang="ts">
	import type { Question, SurveyQuestions } from '$lib/common';
	import QContainer from '$lib/QContainer.svelte';

	let questions: SurveyQuestions = [];

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
	}

	function removeQuestion(uuid: string) {
		questions = questions.filter((q) => q.uuid !== uuid);
	}
</script>
