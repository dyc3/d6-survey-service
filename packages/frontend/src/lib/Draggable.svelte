<script lang='ts'>
	export let index: number;
	import { createEventDispatcher } from "svelte";
  	let hovering: number | boolean | null = null;

	//create event dispatcher
	const dispatch = createEventDispatcher();

  	const drop = (event: DragEvent & { currentTarget: EventTarget & HTMLDivElement; }, target: number) => {
		if(event.dataTransfer !== null){
			event.dataTransfer.dropEffect = 'move'; 
			const oldIndex = parseInt(event.dataTransfer.getData("text/plain"));
			const newIndex = target;
			//dispatch event
			dispatch('move', {oldIndex, newIndex});
		

			hovering = false;
		}
	}

  const dragstart = ( event: DragEvent & { currentTarget: EventTarget & HTMLDivElement; }, i: number) => {
	if(event.dataTransfer !== null){
		event.dataTransfer.effectAllowed = 'move';
		event.dataTransfer.dropEffect = 'move';
		const start = i;
		event.dataTransfer.setData('text/plain', start.toString());
	}
  }
</script>


<!-- <section on:mousedown={onMouseDown} style="top: {top}px;" class="draggable">
	<slot></slot>
</section>

<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} /> -->

<div 
class='draggable'
draggable={true}
on:dragstart={event => dragstart(event, index)}
on:drop|preventDefault={event => drop(event, index)}
on:dragover={() => false}
on:dragenter={() => hovering = index}
class:is-active={hovering === index}
>
	<slot></slot>
</div>

<style>
	.draggable {
		user-select: none;
        position: relative;
	}
</style>