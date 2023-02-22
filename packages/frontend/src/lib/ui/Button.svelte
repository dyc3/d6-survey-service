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

	@import 'main.scss';

	button {
		cursor: pointer;
		display: grid;
		border-radius: 3px;
		place-items: center;
		background-color: #fff;
		color: $main-blue;
		border: none;
	}

	button:active {
		background: $main-gradient;
		color: #fff;
		position: relative;
		top: 1px;
	}

	[aria-pressed='true'] {
		background: $main-gradient;
		color: #fff;
	}

	.border-wrap{
		display: inline-block;
 	 	padding: 1px;
		border-radius: 5px;
	}

	.wrapkind-default, .wrapkind-primary{
		background: $main-gradient;
	}
	.wrapkind-danger{
		background: $main-red;
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
		color: $main-blue;
	}

	.kind-primary:active {
		background: $main-gradient;
		color: #fff;
	}

	.kind-danger{
		background-color: #fff;
		color: $main-red;
	}

	.kind-danger:active {
		background: $main-red;
		color: #fff;
	}
</style>
