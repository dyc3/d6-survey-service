<script lang="ts">
	import Button from './Button.svelte';

	export let name = '';
	export let multiline = false;
	export let placeholder = '';
	export let value = '';
	export let disabled = false;
	export let password = false;

	export let copyable = false;

	async function copyToClipboard() {
		await navigator.clipboard.writeText(value);
	}

	let multilineElement: HTMLTextAreaElement;
	function handleKeydown() {
		multilineElement.style.height = '0px';
		multilineElement.style.height = multilineElement.scrollHeight + 'px';
	}
</script>

<span class="textbox-outer">
	{#if multiline}
		<textarea
			class="textbox"
			{name}
			{placeholder}
			{disabled}
			bind:value
			on:keydown={handleKeydown}
			bind:this={multilineElement}
			on:change
		/>
	{:else if password}
		<!-- Frustratingly, `type` can't be dynamic if 2 way binding is used. -->
		<input class="textbox" {name} type="password" {placeholder} {disabled} bind:value on:change />
	{:else}
		<input class="textbox" {name} type="text" {placeholder} {disabled} bind:value on:change />
	{/if}

	{#if copyable}
		<div class="copy">
			<Button size="small" on:click={copyToClipboard}>Copy</Button>
		</div>
	{/if}
</span>

<style lang="scss">
	@import 'variables';

	.textbox-outer {
		display: flex;
		flex-direction: row;
		width: 100%;
		align-items: center;
		margin: var(--margin, 5px);
	}

	.textbox {
		border: 3px solid $color-default;
		border-radius: 3px;
		flex-grow: 1;
		background-color: $color-bg;
		box-sizing: border-box;
		color: $color-default;
		padding: 0.5em;
		font-size: $main-font-size;
		font-family: inherit;
		overflow-y: hidden;
	}

	.textbox:disabled {
		opacity: 0.5;
	}

	.copy {
		margin-left: 5px;
	}
</style>
