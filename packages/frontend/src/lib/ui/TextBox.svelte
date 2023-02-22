<script lang="ts">
	export let multiline = false;
	export let placeholder = '';
	export let value = '';
	export let disabled = false;
</script>

{#if multiline}
	<textarea rows="4" cols="50" {placeholder} {disabled} bind:value />
{:else}
	<input type="text" {placeholder} {disabled} bind:value />
{/if}

<style lang="scss">
	@import 'main.scss';
	input,
	textarea {
		border: 3px solid $main-blue;
		border-radius: 3px;
		background-color: #fff;
		min-width: 70%;
		font-family: $main-font;
		font-weight: 700;
		color: $main-blue;
		padding: 0.5em;
	}
</style>
