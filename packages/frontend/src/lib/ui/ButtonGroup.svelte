<script lang="ts">
	import Button from './Button.svelte';

	/**
	 * Whether the button group should show vertically or horizontally.
	 */
	export let orientation: 'horizontal' | 'vertical';
	export let buttons: string[];
	export let selected: number | undefined = undefined;

	function select(i: number) {
		selected = i;
	}
</script>

<div class="buttonGroup {orientation}">
	{#each buttons as button, i}
		<Button toggleable={true} pressed={selected == i} on:click={() => select(i)}>{button}</Button>
	{/each}
</div>

<style lang="scss">

	.buttonGroup {
		display: flex;
		justify-content: space-around;
		align-items: center;
	}

	.horizontal {
		flex-direction: row;
	}

	.vertical {
		flex-direction: column;
	}
</style>
