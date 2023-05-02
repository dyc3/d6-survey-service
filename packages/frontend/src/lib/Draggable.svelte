<script lang="ts">
	export let index: number;
	import { createEventDispatcher } from 'svelte';
	let hovering: number | boolean | null = null;
	let hoveringChild = false;

	//create event dispatcher
	const dispatch = createEventDispatcher();

	function drop(
		event: DragEvent & { currentTarget: EventTarget & HTMLDivElement },
		target: number
	) {
		event.preventDefault();
		if (event.dataTransfer !== null) {
			event.dataTransfer.dropEffect = 'move';
			const oldIndex = parseInt(event.dataTransfer.getData('text/plain'));
			const newIndex = target;
			//dispatch event for parent to handle
			dispatch('move', { oldIndex, newIndex });

			hovering = false;
			hoveringChild = false;
		}
	}

	function dragstart(
		event: DragEvent & { currentTarget: EventTarget & HTMLDivElement },
		i: number
	) {
		if (event.dataTransfer !== null && event.target !== null) {
			// kind of cheap way to do it but basically if the dataTransfer to drag start has an item in the list
			// then we know it comes from the draggable icon since anywhere else has an empty list.
			if (event.dataTransfer.items.length > 0) {
				event.dataTransfer.effectAllowed = 'move';
				event.dataTransfer.dropEffect = 'move';
				const start = i;
				event.dataTransfer.setData('text/plain', start.toString());
			}
		}
	}
</script>

<div
	class="draggable {hovering || hoveringChild ? 'hovering' : ''}"
	on:dragstart={(event) => dragstart(event, index)}
	on:drop={(event) => drop(event, index)}
	on:dragover={(event) => {
		event.preventDefault();
		hoveringChild = true;
	}}
	on:dragenter={() => {
		hovering = index;
	}}
	on:dragleave={() => {
		hoveringChild = false;
		if (!hoveringChild) {
			hovering = false;
		}
	}}
>
	<div
		id="draggable-icon-container"
		draggable={true}
		on:dragstart={(event) => {
			//fire the drag event but with the parent
			const parent = event.currentTarget.parentElement;
			if (parent !== null) {
				let dataList = event.dataTransfer?.items;
				if (dataList !== null) {
					dataList?.add('draggable-icon', 'text/plain');
				}
				parent.dispatchEvent(
					new DragEvent('dragstart', {
						bubbles: true,
						cancelable: true,
						composed: true,
						dataTransfer: event.dataTransfer
					})
				);
			}
		}}
	>
		<p class="draggable-icon" id="draggable-icon">=</p>
	</div>
	<slot />
</div>

<style lang="scss">
	@import './ui/variables';

	.draggable-icon {
		margin: 0;
		padding: 0;
		font-size: 1.5rem;
		font-weight: bold;
		color: $color-blue;
	}

	.hovering {
		background-color: #3273dc22;
	}
</style>
