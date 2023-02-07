<script lang="ts">
	export let editmode = false;
	export let multiline = false;
	let response = '';
	export let prompt = '';
</script>

<div>
	<div>
		{#if editmode}
			<input bind:value={prompt} />
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	{#if multiline}
		<textarea bind:value={response} disabled={editmode} />
	{:else}
		<input bind:value={response} disabled={editmode} />
	{/if}
</div>
