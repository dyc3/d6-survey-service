<script lang="ts">
	import '$lib/main.scss';
</script>

<div class="container">
	<h1>Survey App</h1>
	<slot />
</div>

<style lang="scss">
	$page-width: 1000px;

	.container {
		max-width: $page-width;
		margin: 0 auto;

		@media screen and (max-width: $page-width) {
			max-width: 100%;
		}
	}
</style>
