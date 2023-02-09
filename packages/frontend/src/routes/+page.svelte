<script lang="ts">
	import QContainer from '$lib/QContainer.svelte';
	import QRating from '$lib/questions/QRating.svelte';
	import QTextInput from '../lib/questions/QTextInput.svelte';
	import type { Question } from '../lib/common';

	let editmode = false;

	const questions: Question[] = [
		{
			type: 'Text',
			content: { prompt: 'What is your name?', description: '', multiline: false }
		},
		{
			type: 'Rating',
			content: { prompt: 'Rate your experience', description: '', max_rating: 10 }
		}
	];
	let selected_question = 0;
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h2>Component Demo</h2>

<div>
	<input type="checkbox" id="editmode" bind:checked={editmode} />
	<label for="editmode">Survey Edit Mode</label>
</div>

<QTextInput prompt="What is your name?" description="" {editmode} />
<QTextInput prompt="Describe your shoes." description="" multiline={true} {editmode} />
<QRating prompt="Rate your experience" description="" max_rating={10} {editmode} />

<h3>QContainer</h3>

<select bind:value={selected_question}>
	{#each questions as q, i}
		<option value={i}>{q.type}</option>
	{/each}
</select>

<QContainer question={questions[selected_question]} {editmode} />
