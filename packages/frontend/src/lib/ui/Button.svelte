<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	/**
	 * Makes the button toggle when clicked.
	 */
	export let toggleable = false;
	/**
	 * Whether the button is currently pressed.
	 */
	export let pressed = false;
	export let size: 'small' | 'normal' | 'large' = 'normal';
	export let kind: 'primary' | 'danger' | 'default' = 'default';

	$: classes = `sz-${size} kind-${kind}`;
	$: wrapclasses = `border-wrap wrapkind-${kind}`
	

	const dispatch = createEventDispatcher();

	function toggle() {
		pressed = !pressed;
	}

	function handleClick(e: Event) {
		toggle();
		dispatch('click', e);
	}
</script>

{#if toggleable}
	<div class={wrapclasses}>
		<button class={classes} aria-pressed={pressed} on:click={handleClick}>
			<slot />
		</button>
	</div>
{:else}
	<div class={wrapclasses}>
		<button class={classes} on:click>
			<slot />
		</button>
	</div>
{/if}

<style lang="scss">
	button {
		cursor: pointer;
		display: grid;
		width: 100%;
		height: 100%;
		border-radius: 3px;
		place-items: center;
		background-color: #fff;
		color: #426881;
		border: none;
	}

	button:active {
		background: rgb(66, 104, 129);
		background: -moz-linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		background: -webkit-linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		background: linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		color: #fff;
		position: relative;
		top: 1px;
	}

	[aria-pressed='true'] {
		background-color: #000;
		color: #fff;
	}

	.border-wrap{
		display: flex;
		position: relative;
 	 	padding: 3px;
		border-radius: 5px;
	}

	.wrapkind-default, .wrapkind-primary{
		background: rgb(66, 104, 129);
		background: -moz-linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		background: -webkit-linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		background: linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
	}
	.wrapkind-danger{
		background: #CD311E;
	}
	.sz-small {
		font-size: 0.8em;
		padding: 0.2em 0.5em;
	}

	.sz-normal {
		font-size: 1em;
		padding: 0.5em 2em;
	}

	.sz-large {
		font-size: 1.4em;
		padding: 0.6em 4em;
	}

	.kind-primary {
		background-color: #fff;
		color: #426881;
	}

	.kind-primary:active {
		background: rgb(66, 104, 129);
		background: -moz-linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		background: -webkit-linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		background: linear-gradient(90deg, rgba(66, 104, 129, 1) 0%, rgba(65, 128, 83, 1) 100%);
		color: #fff;
	}

	.kind-danger{
		background-color: #fff;
		color: #CD311E;
	}

	.kind-danger:active {
		background: #CD311E;
		color: #fff;
	}
</style>
