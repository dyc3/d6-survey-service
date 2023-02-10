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
	export let kind: 'primary' | 'default' = 'default';

	$: classes = `sz-${size} kind-${kind}`;

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
	<button class={classes} aria-pressed={pressed} on:click={handleClick}>
		<slot />
	</button>
{:else}
	<button class={classes} on:click>
		<slot />
	</button>
{/if}

<style lang="scss">
	button {
		cursor: pointer;
		border: #000 3px solid;
		border-radius: 5px;
		background-color: #fff;
	}

	button:active {
		background-color: #000;
		color: #fff;
		position: relative;
		top: 1px;
	}

	[aria-pressed='true'] {
		background-color: #000;
		color: #fff;
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
		background-color: #000;
		color: #fff;
	}

	.kind-primary:active {
		background-color: #fff;
		color: #000;
	}
</style>
