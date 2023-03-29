<script lang="ts">
	import Button from './Button.svelte';

	interface Item<T> {
		label: string;
		value?: T;
	}

	type T = $$Generic;

	/**
	 * Whether the button group should show vertically or horizontally.
	 */
	export let orientation: 'horizontal' | 'vertical';
	export let buttons: Item<T>[];
	export let forceSelection: boolean;
	let selected: Set<number> = new Set();
	export let role: string | undefined = undefined;
	export let size: 'small' | 'normal' | 'large' = 'normal';
	export let multiple = false;
	export let selectedIndexes: number[] = [];
	export let selectedValues: T[] = [];

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

	$: updateSelection(selected);

	function updateSelection(idxs: typeof selected) {
		selectedIndexes = Array.from(idxs);
		let values = selectedIndexes.map((i) => buttons[i].value).filter((v) => v !== undefined);
		if (values.length !== selectedIndexes.length) {
			selectedValues = [];
		} else {
			selectedValues = values as T[];
		}
		console.log(selectedIndexes, selectedValues);
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
