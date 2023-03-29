<script lang="ts">
	import Button from './Button.svelte';

	interface Item<T> {
		label: string;
		value?: T;
	}

	/**
	 * Whether the button group should show vertically or horizontally.
	 */
	export let orientation: 'horizontal' | 'vertical';
	export let buttons: Item<unknown>[];
	export let forceSelection: boolean;
	let selected: Set<number> = new Set();
	export let role: string | undefined = undefined;
	export let size: 'small' | 'normal' | 'large' = 'normal';
	export let multiple = false;

	function select(i: number) {
		if (selected.has(i)) {
			if (forceSelection) {
				return;
			}
			selected.delete(i);
			selected = selected;
		} else {
			if (multiple) {
				selected.add(i);
				selected = selected;
			} else {
				selected = new Set([i]);
			}
		}
	}
</script>

<div class="button-group {orientation}">
	{#each buttons as button, i}
		<Button
			toggleable={true}
			{size}
			inButtonGroup={true}
			pressed={selected.has(i)}
			on:click={() => select(i)}
			{role}>{button.label}</Button
		>
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
		align-items: stretch;
	}
</style>
