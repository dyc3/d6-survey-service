<script lang="ts">
	import TextBox from '$lib/ui/TextBox.svelte';
	import Button from '$lib/ui/Button.svelte';

	export let editmode = false;
	let response = 0;
	export let prompt: string;
	export let description: string;
	export let max_rating = 10;

	export let minText = "low";
	export let maxText = "high";
</script>

<div>
	<div>
		{#if editmode}
			<span>On a scale of 1- <input bind:value={max_rating} type="number" /> ...</span>
			<span>Where 1 is <input bind:value={minText}/> and {max_rating} is <input bind:value={maxText}/></span>
		{:else}
			<span>On a scale of 1-{max_rating}...</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={prompt} />
		{:else}
			<span>{prompt}</span>
		{/if}
	</div>

	<div>
		{#if editmode}
			<TextBox bind:value={description} />
		{:else}
			<span>{description}</span>
		{/if}
	</div>

	<div style="width: max-content;">
			{#each Array(max_rating) as value, i}
				<Button size="small">{i+1}</Button>
				{/each}
			<br>
			<span class="align-rating-text">
			<span>{minText}</span>
			<span>{maxText}</span>
			</span>
	</div>
</div>

<style>
	.align-rating-text {
		display: flex;
		justify-content: space-between;
	}
</style>
