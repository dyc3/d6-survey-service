<script>
	export let top = 0;
    let oldTop = 0;
	
	let moving = false;
	
	function onMouseDown() {
		moving = true;
        oldTop = top;
	}
	
	/**
	 * @param {{ movementY: number; }} e
	 */
	function onMouseMove(e) {
		if (moving) {
			top += e.movementY;
		}
	}
	
	function onMouseUp() {
		moving = false;
        top = oldTop;
	}
</script>


<!-- <section on:mousedown={onMouseDown} style="top: {top}px;" class="draggable">
	<slot></slot>
</section>

<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} /> -->

<section draggable="true" style="top: {top}px;" class="draggable">
	<slot></slot>
</section>

<style>
	.draggable {
		user-select: none;
        position: relative;
	}
</style>