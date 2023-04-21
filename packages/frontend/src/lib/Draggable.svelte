<script lang="ts">
	export let index: number;
	import { createEventDispatcher } from 'svelte';
	import { add_classes, toggle_class } from 'svelte/internal';
	let hovering: number | boolean | null = null;

	//create event dispatcher
	const dispatch = createEventDispatcher();

	const drop = (
		event: DragEvent & { currentTarget: EventTarget & HTMLDivElement },
		target: number
	) => {
		event.preventDefault();
		console.log('brh');
		if (event.dataTransfer !== null) {
			event.dataTransfer.dropEffect = 'move';
			const oldIndex = parseInt(event.dataTransfer.getData('text/plain'));
			const newIndex = target;
			//dispatch event for parent to handle
			dispatch('move', { oldIndex, newIndex });

			hovering = false;
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
	class="draggable {hovering === index ? 'hovering' : ''}"
	draggable={true}
	on:dragstart={(event) => dragstart(event, index)}
	on:drop={(event) => drop(event, index)}
	on:dragover={() => false}
	on:dragenter={() => {
		hovering = index;
	}}
	on:dragleave={() => {
		hovering = false;
	}}
>
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
