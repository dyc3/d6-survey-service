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

	$: classes = `kind-${kind} sz-${size}`;

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
	<button class={classes} on:click={handleClick} aria-pressed={pressed}>
		<div class="surface">
			<slot />
		</div>
	</button>
{:else}
	<button class={classes} on:click>
		<div class="surface">
			<slot />
		</div>
	</button>
{/if}

<style lang="scss">
	@import 'main.scss';
	$btn-border-size: 3px;

	button {
		cursor: pointer;
		display: inline-block;
		padding: $btn-border-size;
		border-radius: 5px;
		border: none;
	}

	.surface {
		background: #fff;
		border-radius: 3px;
		font-family: $main-font;
		font-weight: 500;
	}

	button:active {
		position: relative;
		top: 1px;
	}

	button:active,
	[aria-pressed='true'] {
		.surface {
			background: transparent;
			color: #fff;
		}
	}

	.sz-small {
		font-size: 1em;
		.surface {
			padding: 0.2em 0.5em;
		}
	}

	.sz-normal {
		font-size: 1.4em;
		.surface {
			padding: 0.5em 2em;
		}
	}

	.sz-large {
		font-size: 1.6em;
		.surface {
			padding: 0.6em 4em;
		}
	}

	.kind-primary {
		background: $main-blue;
		color: $main-blue;
	}

	.kind-default {
		background: $main-gradient;
		color: $main-blue;
	}

	.kind-danger {
		background: $main-red;
		color: $main-red;
	}
</style>
