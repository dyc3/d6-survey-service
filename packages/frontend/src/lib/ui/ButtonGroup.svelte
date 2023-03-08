<script lang="ts">
	import Button from './Button.svelte';

	/**
	 * Whether the button group should show vertically or horizontally.
	 */
	export let orientation: 'horizontal' | 'vertical';
	export let buttons: string[];
	export let forceSelection: boolean;
	export let selected: number | undefined = undefined;

	function select(i: number) {
		if(selected == i)
			if(forceSelection){
			selected = 0;
			}
			else
				selected = undefined;
		else
			selected = i;
	}
</script>

<div class="button-group {orientation}">
	{#each buttons as button, i}
		<Button toggleable={true} inButtonGroup={true} pressed={selected == i} on:click={() => select(i)}>{button}</Button>
	{/each}
</div>

<style lang="scss">
	@import 'variables.scss';

	.button-group {
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
