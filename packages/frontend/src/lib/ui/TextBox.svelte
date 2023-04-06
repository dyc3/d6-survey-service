<script lang="ts">
	export let name = '';
	export let multiline = false;
	export let placeholder = '';
	export let value = '';
	export let disabled = false;

	let multilineElement : HTMLTextAreaElement ;
	function handleKeydown() {
		multilineElement.style.height = "0px";
		multilineElement.style.height = multilineElement.scrollHeight + "px";
	}
</script>

{#if multiline}
	<textarea class="textbox" {name} {placeholder} {disabled} bind:value on:keydown={handleKeydown} bind:this={multilineElement} on:change />
{:else}
	<input class="textbox" {name} type="text" {placeholder} {disabled} bind:value on:change />
{/if}

<style lang="scss">
	@import 'variables';

	.textbox {
		border: 3px solid $color-default;
		border-radius: 3px;
		background-color: $color-bg;
		min-width: 70%;
		color: $color-default;
		padding: 0.5em;
		font-size: $main-font-size;
		font-family: inherit;
		margin: var(--margin, 5px);
		overflow-y: hidden;
	}

	.textbox:disabled {
		opacity: 0.5;
	}
</style>
