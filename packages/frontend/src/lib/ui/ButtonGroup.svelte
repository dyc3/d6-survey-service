<script lang="ts">
	import Button from './Button.svelte';

	/**
	 * Whether the button group should show vertically or horizontally.
	 */
	export let orientation: 'horizontal' | 'vertical';
	export let buttons: string[];
	export let forceSelection: boolean;
	export let selected: number | undefined = undefined;
	export let role: string | undefined = undefined;
	export let size: 'small' | 'normal' | 'large' = 'normal';

	function select(i: number) {
		if(selected == i)
			if(forceSelection){
				return;
			}
			else
				selected = undefined;
		else
			selected = i;
	}
</script>

<div class="button-group {orientation}">
	{#each buttons as button, i}
		<Button toggleable={true} size={size} inButtonGroup={true} pressed={selected == i} on:click={() => select(i)} role={role}>{button}</Button>
	{/each}
</div>

<style lang="scss">
	@import 'variables';

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
