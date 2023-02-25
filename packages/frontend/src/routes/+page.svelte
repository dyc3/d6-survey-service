<script lang="ts">
	import QContainer from '$lib/QContainer.svelte';
	import QRating from '$lib/questions/QRating.svelte';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import QTextInput from '../lib/questions/QTextInput.svelte';
	import type { Question } from '../lib/common';
	import QMultipleChoice from '$lib/questions/QMultipleChoice.svelte';

	let editmode = false;

	const questions: Question[] = [
		{
			type: 'Text',
			content: { prompt: 'What is your name?', description: '', multiline: false }
		},
		{
			type: 'Rating',
			content: { prompt: 'Rate your experience', description: '', max_rating: 10 }
		},
		{
			type: 'MultipleChoice',
			content: {
				prompt: 'Do you like tacos?',
				description: '',
				multiple: false,
				choices: [
					{
						uuid: '1',
						text: 'Yes'
					},
					{
						uuid: '2',
						text: 'No'
					}
				]
			}
		}
	];
	let selected_question = 0;
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h2>Component Demo</h2>

<h3>UI Primitives</h3>

Buttons
<div>
	<Button>Default</Button>
	<Button kind="primary">Primary</Button>
	<Button kind="danger">Danger Button</Button>
</div>
<div>
	<Button size="small">Small</Button>
	<Button size="normal">Normal</Button>
	<Button size="large">Large</Button><br />
</div>
<div>
	<Button toggleable={true}>Toggle Button</Button>
</div>

Textboxes
<div>
	<TextBox placeholder="placeholder" />
</div>
<div>
	<TextBox placeholder="placeholder" multiline={true} />
</div>

<h3>Questions</h3>

<div>
	<input type="checkbox" id="editmode" bind:checked={editmode} />
	<label for="editmode">Survey Edit Mode</label>
</div>

<QTextInput prompt="What is your name?" description="" {editmode} />
<QTextInput prompt="Describe your shoes." description="" multiline={true} {editmode} />
<QRating prompt="Rate your experience" description="" max_rating={10} {editmode} />
<QMultipleChoice prompt="Do you like tacos?" description="" {editmode} />

<h3>QContainer</h3>

<select bind:value={selected_question}>
	{#each questions as q, i}
		<option value={i}>{q.type}</option>
	{/each}
</select>

<QContainer question={questions[selected_question]} {editmode} />
