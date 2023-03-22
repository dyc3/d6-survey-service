<script lang="ts">
	import TextBox from '$lib/ui/TextBox.svelte';
	import Button from '$lib/ui/Button.svelte';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';
	import Container from '$lib/ui/Container.svelte';

	export let editmode = false;
	let response = 0;
	export let prompt: string;
	export let description: string;
	export let max_rating = 10;
	export let required = false;

	export let minText = 'low';
	export let maxText = 'high';

	let group_selected: number | undefined = undefined;
</script>

<Container>

	{#if required}
			<span class = "required">*</span>
	{/if}

	<div>
		{#if editmode}
			<span>On a scale of 1- <input bind:value={max_rating} type="number" on:change /> ...</span>
			<span
				>Where 1 is <input bind:value={minText} on:change /> and {max_rating} is
				<input bind:value={maxText} on:change /></span
			>
		{:else}
			<span class="prompt-text">On a scale of 1-{max_rating}...</span>
		{/if}
	</div>

	<div class="text-box-container">
		{#if editmode}
			<TextBox placeholder="Insert prompt..." bind:value={prompt} />
		{:else}
			<span class="prompt-text">{prompt}</span>
		{/if}
	</div>

	<div class="text-box-container">
		{#if editmode}
			<TextBox placeholder="Insert description..." bind:value={description} />
		{:else}
			<span class="description-text">{description}</span>
		{/if}
	</div>

	<div style="width: max-content;">
		<ButtonGroup
			orientation="horizontal"
			size="small"
			buttons={Array.apply('', Array(max_rating)).map(function (x, i) {
				return (i + 1).toString();
			})}
			forceSelection={false}
			bind:selected={group_selected}
		/>
		<div class="align-rating-text">
			<span class="description-text">{minText}</span>
			<span class="description-text">{maxText}</span>
		</div>
	</div>
</Container>

<style lang="scss">
	@import '../ui/variables';

	.prompt-text {
		font-weight: bold;
		font-size: $bold-font-size;
		color: $color-blue;
	}

	.text-box-container {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
	}

	.description-text {
		font-size: $main-font-size;
		color: $color-blue;
	}

	.align-rating-text {
		display: flex;
		justify-content: space-between;
	}

	.required{
		color: red;
		position: absolute;
		right: 2.5%;
		top: 0;
	}
</style>
