<script lang="ts">
	import QContainer from '$lib/QContainer.svelte';
	import Button from '$lib/ui/Button.svelte';
	import QTextInput from '../lib/questions/QTextInput.svelte';

	let editmode = false;
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h2>Component Demo</h2>

<h3>UI Primitives</h3>

<div>
	<Button>Default</Button>
</div>
<div>
	<Button size="small">Small</Button>
	<Button size="normal">Normal</Button>
	<Button size="large">Large</Button><br />
</div>
<div>
	<Button toggleable={true}>Toggle Button</Button>
</div>

<h3>Questions</h3>

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
		content: { prompt: 'What is your name?', description: '', multiline: false }
	}}
	{editmode}
/>
