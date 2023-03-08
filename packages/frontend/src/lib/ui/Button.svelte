<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let type: 'button' | 'submit' | 'reset' | undefined = undefined;
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
	export let inButtonGroup = false;

	$: classes = `kind-${kind} sz-${size}`;

	const dispatch = createEventDispatcher();

	function toggle() {
		pressed = !pressed;
		dispatch('message', {
			text: 'toggled'
		})
	}

	function handleClick(e: Event) {
		if (!inButtonGroup){
		toggle();
		}
		dispatch('click', e);
	}
</script>

{#if toggleable}
	<button {type} class={classes} on:click={handleClick} aria-pressed={pressed}>
		<div class="surface">
			<slot />
		</div>
	</button>
{:else}
	<button {type} class={classes} on:click>
		<div class="surface">
			<slot />
		</div>
	</button>
{/if}

<style lang="scss">
	@import 'variables';

	$btn-border-size: 3px;

	button {
		cursor: pointer;
		display: inline-block;
		padding: $btn-border-size;
		border-radius: 5px;
		border: none;
	}

	.surface {
		background: $color-surface;
		border-radius: 3px;
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
			color: $color-surface;
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

	.kind-default {
		background: $gradient-default;
		color: $color-default;
	}

	.kind-primary {
		background: $color-primary;
		color: $color-primary;

		.surface {
			background: $color-primary;
			color: $color-surface;
		}
	}

	.kind-primary:active {
		.surface {
			color: #c4c4c4;
		}
	}

	.kind-danger {
		background: $gradient-danger;
		color: $color-danger;
	}
</style>
