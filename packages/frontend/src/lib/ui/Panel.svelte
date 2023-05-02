<script lang="ts">
	export let border = false;
</script>

<div class="panel" class:with-border={border}>
	<slot />
</div>

<style lang="scss">
	@import './variables';

	.panel {
		width: 100%;
		padding: 20px 30px;
		height: max-content;
		box-sizing: border-box;
	}

	.with-border {
		border: $color-blue 3px solid;
		border-radius: 10px;
	}
</style>
