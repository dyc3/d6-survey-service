<script lang="ts">
	import QContainer from '$lib/QContainer.svelte';
	import QRating from '$lib/questions/QRating.svelte';
	import Button from '$lib/ui/Button.svelte';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import QTextInput from '$lib/questions/QTextInput.svelte';
	import type { Question } from '$lib/common';
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

	let selected1: number[] = [];
	let selected2: string[] = [];

	let loading = true;
</script>

<h1>Component Demo</h1>
<p><a href="https://kit.svelte.dev">Svelte Kit docs</a></p>

<h3>UI Primitives</h3>

Buttons
<div>
	<Button>Default</Button>
	<Button disabled={true}>Disabled</Button>
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
<input type="checkbox" bind:checked={loading} />
<div>
	<Button {loading}>Loading Default</Button>
	<Button kind="primary" {loading}>Loading Primary</Button>
	<Button kind="danger" {loading}>Loading Danger</Button>
</div>
<div>
	<h3>Single selection</h3>
	<p>Selected: {selected1}</p>
	<ButtonGroup
		orientation="horizontal"
		buttons={['Button 1', 'Button 2', 'Button 3']}
		forceSelection={false}
		bind:selected={selected1}
	/>

	<h3>Multiple selection, with values</h3>
	<p>Selected: {selected2}</p>
	<ButtonGroup
		orientation="horizontal"
		buttons={[
			{ label: 'Button 1', value: 'apple' },
			{ label: 'Button 2', value: 'orange' },
			{ label: 'Button 3', value: 'banana' }
		]}
		forceSelection={false}
		multiple={true}
		bind:selected={selected2}
	/>
</div>

Textboxes
<div>
	<TextBox placeholder="placeholder" />
</div>
<div>
	<TextBox placeholder="placeholder" multiline={true} />
</div>
<div>
	<TextBox placeholder="readonly multiline with disabled" multiline={true} disabled={true} />
</div>
<div>
	<TextBox placeholder="readonly with disabled" multiline={false} disabled={true} />
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
<QContainer question={questions[selected_question]} {editmode} />
<QContainer question={questions[selected_question]} {editmode} required={true} />
