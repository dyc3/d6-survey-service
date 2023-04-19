<script lang="ts">
	import Button from './Button.svelte';

	interface Item<T> {
		label: string;
		value: T;
	}

	type T = $$Generic;

	type SelectionValue<T> = typeof buttons extends string[] ? number : NonNullable<T>;

	/**
	 * Whether the button group should show vertically or horizontally.
	 */
	export let orientation: 'horizontal' | 'vertical';
	export let buttons: (string | Item<T>)[];
	export let forceSelection: boolean;
	export let selected: SelectionValue<T>[] = [];
	export let role: string | undefined = undefined;
	export let size: 'small' | 'normal' | 'large' = 'normal';
	export let multiple = false;

	function isItem<T>(item: string | Item<T>): item is Item<T> {
		return typeof item !== 'string';
	}

	function getSelectionValue<T>(i: number): SelectionValue<T> {
		let item = buttons[i];
		if (isItem(item)) {
			return item.value as SelectionValue<T>;
		} else {
			return i as SelectionValue<T>;
		}
	}

	function select(i: number) {
		let value: SelectionValue<T> = getSelectionValue(i);
		let idx = selected.indexOf(value);
		if (idx >= 0) {
			if (forceSelection) {
				return;
			}
			selected.splice(idx, 1);
			selected = selected;
		} else {
			if (multiple) {
				selected.push(value);
				selected = selected;
			} else {
				selected = [value];
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
			pressed={selected.includes(getSelectionValue(i))}
			on:click={() => select(i)}
			{role}
			--margin="2px"
		>
			{#if isItem(button)}
				{button.label}
			{:else}
				{button}
			{/if}
		</Button>
	{/each}
</div>

<style lang="scss">
	@import 'variables';

	.button-group {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.button-wrapper {
		margin: 2px;
	}

	.horizontal {
		flex-direction: row;
	}

	.vertical {
		flex-direction: column;
		align-items: stretch;
	}
</style>
