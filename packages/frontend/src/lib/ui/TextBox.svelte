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
	input,
	textarea {
		border: #000 3px solid;
		border-radius: 5px;
		background-color: #fff;
	}
</style>
