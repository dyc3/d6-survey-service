<script lang="ts">
	export let name = '';
	export let multiline = false;
	export let placeholder = '';
	export let value = '';
	export let disabled = false;
</script>

{#if multiline}
	<textarea {name} {placeholder} {disabled} bind:value on:change />
{:else}
	<input {name} type="text" {placeholder} {disabled} bind:value on:change />
{/if}

<style lang="scss">
	@import 'variables';

	input,
	textarea {
		border: 3px solid $color-default;
		border-radius: 3px;
		background-color: #fff;
		min-width: 70%;
		font-weight: 700;
		color: $color-default;
		padding: 0.5em;
	}
</style>
