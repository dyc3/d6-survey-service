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
	@use 'sass:map';
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
		-webkit-background-clip: text;
		clip-path: inset(2px);
		padding: 2px;
		width: auto;
		-webkit-background-clip: text; /* stylelint-disable-line property-no-vendor-prefix */
	}

	$sizes: (
		small: (
			font-size: 1em,
			padding: 0.2em 0.5em
		),
		normal: (
			font-size: 1.4em,
			padding: 0.5em 2em
		),
		large: (
			font-size: 1.6em,
			padding: 0.6em 4em
		)
	);

	@each $size, $props in $sizes {
		.sz-#{$size} {
			font-size: map.get($props, font-size);

			.surface {
				padding: map.get($props, padding);
			}
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
			background: map.get($props, bg);

			@supports (not (background-clip: text)) or (not (-webkit-background-clip: text)) {
				color: map.get($props, color);
			}

			@supports (background-clip: text) or (-webkit-background-clip: text) {
				.subsurface {
					background: map.get($props, bg);
					background-clip: text;
					-webkit-background-clip: text; /* stylelint-disable-line property-no-vendor-prefix */
					color: transparent;
				}
			}
		}
	}

	button:hover {
		transform: scale(1.02);
	}

	button:active {
		transform: scale(0.99);
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
	}
</style>
