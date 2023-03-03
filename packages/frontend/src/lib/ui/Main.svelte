<div class="main-layout">
	<slot />
</div>

<style lang="scss">
	.main-layout {
		justify-content: center;
		align-items: center;

		padding: 20px 30px 20px 30px;
		margin: auto;

		width: max-content;

		border: #000 3px solid;
		border-radius: 4px;

		line-height: 2;
		color: black;
	}
</style>
