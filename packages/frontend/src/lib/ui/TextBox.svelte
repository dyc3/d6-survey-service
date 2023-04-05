<script lang="ts">
	import Button from "./Button.svelte";

	export let name = '';
	export let multiline = false;
	export let placeholder = '';
	export let value = '';
	export let disabled = false;

	export let copy = false;

	async function copyToClipboard(){
		await navigator.clipboard.writeText(value);
	}
</script>

{#if multiline}
	<textarea {name} {placeholder} {disabled} bind:value on:change />
{:else}
	<input {name} type="text" {placeholder} {disabled} bind:value on:change />
{/if}

{#if copy}
	<Button on:click={copyToClipboard}>Copy to Clipboard</Button>
{/if}

<style lang="scss">
	@import 'variables';

	input,
	textarea {
		border: 3px solid $color-default;
		border-radius: 3px;
		background-color: #fff;
		min-width: 70%;
		color: $color-default;
		padding: 0.5em;
		font-size: $main-font-size;
		font-family: inherit;
	}
</style>
