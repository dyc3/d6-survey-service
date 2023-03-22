<div class="main-layout">
	<slot />
</div>

<style lang="scss">
	@import 'variables';

	.main-layout {
		display: flex;
		position: relative;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 20px 30px;
		margin: auto;
		min-height: fit-content;
		width: max-content;
		border: $color-blue 3px solid;
		border-radius: 10px;
		line-height: 2;
		color: black;
	}
</style>
