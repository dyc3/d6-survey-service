<script lang="ts">
    export let editmode = false;
    export let prompt: string;
    export let description: string;
    import Button from "$lib/ui/Button.svelte";
    import TextBox from "$lib/ui/TextBox.svelte";

    let options = ["Option 1", "Option 2"];

    function addOptions(){
        // work in progress
        // options = [...options, "Option " + options.length + 1];
    }

</script>

<div>
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

	<div>
        {#if editmode}
            <TextBox bind:value={options[0]} />
        {:else}
            <Button>{options[0]}</Button>
        {/if}
    </div>
    <div>
        {#if editmode}
            <TextBox bind:value={options[1]} />
        {:else}
            <Button>{options[1]}</Button>
        {/if}
    </div>
    <div>
        
    </div>
    <div>
        {#if editmode}
        <span>+</span>
        {:else}
        <Button on:click={addOptions}>
        +
        </Button>
        {/if}
    </div>
</div>
