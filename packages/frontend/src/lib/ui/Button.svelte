<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let type: 'button' | 'submit' | 'reset' | undefined = undefined;
	export let role: string | undefined = undefined;
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
	}

	function handleClick(e: Event) {
		if (toggleable && !inButtonGroup) {
			toggle();
		}
		dispatch('click', e);
	}
</script>

<button
	{type}
	class={classes}
	on:click={handleClick}
	aria-pressed={toggleable ? pressed : undefined}
	{role}
>
	<div class="surface">
		<span class="subsurface">
			<slot />
		</span>
	</div>
</button>

<style lang="scss">
	@import 'variables';

	$btn-border-size: 3px;
	$transition-duration: 0.1s;

	button {
		cursor: pointer;
		display: inline-block;
		padding: $btn-border-size;
		border-radius: 5px;
		border: none;

		transition: all $transition-duration ease-in-out;

		* {
			transition: all $transition-duration ease-in-out;
		}
	}

	.surface {
		background: $color-surface;
		border-radius: 3px;
		font-weight: 500;
	}

	.subsurface {
		background-clip: text;
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

	$kinds: (
		default: (
			bg: $gradient-default,
			color: $color-default
		),
		primary: (
			bg: $gradient-primary,
			color: $color-primary
		),
		danger: (
			bg: $gradient-danger,
			color: $color-danger
		)
	);

	@each $kind, $props in $kinds {
		.kind-#{$kind} {
			background: map-get($props, bg);
			@supports not (background-clip: text) {
				color: map-get($props, color);
			}

			@supports (background-clip: text) {
				.subsurface {
					background: text map-get($props, bg);
					color: transparent;
				}
			}
		}
	}

	button:hover {
		.surface {
			background: linear-gradient(
				90deg,
				rgb(255 255 255 / 0%) 0%,
				$color-surface 20%,
				$color-surface 80%,
				rgb(255 255 255 / 0%) 100%
			);
		}
	}

	button:active {
		position: relative;
		top: 1px;
		transition-duration: 0;

		* {
			transition-duration: 0;
		}
	}

	button:active,
	[aria-pressed='true'] {
		.surface {
			background: transparent;
			color: $color-surface;
		}

		.subsurface {
			color: inherit;
		}

		&:hover {
			.surface {
				background: linear-gradient(
					90deg,
					$color-surface 0%,
					rgb(255 255 255 / 0%) 20%,
					rgb(255 255 255 / 0%) 80%,
					$color-surface 100%
				);
			}
		}
	}
</style>
