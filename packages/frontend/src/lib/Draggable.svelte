<script lang="ts">
	export let index: number;
	import { createEventDispatcher } from 'svelte';
	import { add_classes, toggle_class } from 'svelte/internal';
	let hovering: number | boolean | null = null;
	let hoveringChild = false;

	//create event dispatcher
	const dispatch = createEventDispatcher();

	const drop = (
		event: DragEvent & { currentTarget: EventTarget & HTMLDivElement },
		target: number
	) => {
		event.preventDefault();
		console.log(event);
		if (event.dataTransfer !== null) {
			event.dataTransfer.dropEffect = 'move';
			const oldIndex = parseInt(event.dataTransfer.getData('text/plain'));
			const newIndex = target;
			//dispatch event for parent to handle
			dispatch('move', { oldIndex, newIndex });

			hovering = false;
			hoveringChild = false;
		}
	};

	const dragstart = (
		event: DragEvent & { currentTarget: EventTarget & HTMLDivElement },
		i: number
	) => {
		if (event.dataTransfer !== null) {
			event.dataTransfer.effectAllowed = 'move';
			event.dataTransfer.dropEffect = 'move';
			const start = i;
			event.dataTransfer.setData('text/plain', start.toString());
		}
	};
</script>

<div
	class="draggable {hovering || hoveringChild ? 'hovering' : ''}"
	draggable={true}
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
	<span />
	<slot />
</div>

<style>
	.draggable {
		display: flex;
		height: fit-content;
		width: fit-content;
	}

	.hovering {
		background-color: #3273dc22;
	}
</style>
