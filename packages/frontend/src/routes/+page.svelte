<script lang="ts">
	import QTextInput from '../lib/questions/QTextInput.svelte';

	let editmode = false;
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h2>Component Demo</h2>

<div>
	<input type="checkbox" id="editmode" bind:checked={editmode} />
	<label for="editmode">Survey Edit Mode</label>
</div>

<QTextInput prompt="What is your name?" {editmode} />
<QTextInput prompt="Describe your shoes." multiline={true} {editmode} />
