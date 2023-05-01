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

<h1>Survey App</h1>
{#if loggedIn}
	<Button on:click={logout}>Log Out</Button>
{/if}

<slot />
