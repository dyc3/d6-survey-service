<script lang="ts">
	import '$lib/main.scss';
	import Button from '../lib/ui/Button.svelte';
	import { jwt } from '../stores';
	import { goto } from '$app/navigation';

	let loggedIn = false;
	jwt.loggedIn.subscribe((value) => {
		loggedIn = value;
	});

	function logout() {
		jwt.logout();
		goto('/login');
	}
</script>

<div class="header-container">
	<h1>Survey App</h1>
	{#if loggedIn}
		<Button on:click={logout}>Log Out</Button>
	{/if}
</div>

<slot />

<style lang="scss">
	.header-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>
