<script lang="ts">
	import QContainer from '$lib/QContainer.svelte';
	import QTextInput from '../lib/questions/QTextInput.svelte';
	import Login from '../lib/login.svelte';
let name = 'worldzz';
	let editmode = false;
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

<h3>QContainer</h3>

<QContainer
	question={{
		type: 'Text',
		content: { prompt: 'What is your name??', description: '', multiline: false }
	}}
	{editmode}
/>
<Login />