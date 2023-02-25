
<div class="mainLayout">
	<slot>
	</slot>
</div>
<style lang="scss">
	.mainLayout {
		// display: flex;
		justify-content: center;
		align-items: center;

		padding: 20px 30px 20px 30px;
        // margin: 100px 200px 100px 200px; // change to center later?
		margin: auto;

		width: max-content;

		border: #000 3px solid;
		border-radius: 4px;
		background-color: #fff;

        line-height: 2;
        color: black;
		font-family: 'DM Sans';
	}
</style>
