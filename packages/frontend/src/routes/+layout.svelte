<script lang="ts">
	import '$lib/main.scss';
	import Button from '../lib/ui/Button.svelte';
	import { browser } from '$app/environment';
	import { jwt } from '../stores';
	import { goto } from '$app/navigation';

	let loggedIn = false;
	if (browser) {
		if (jwt.get() !== undefined) {
			loggedIn = true;
		}
	}

	function logout() {
		jwt.logout();
		loggedIn = false;
		goto('/login');
	}
</script>

<h1>Survey App</h1>
{#if loggedIn}
	<Button on:click={logout}>Log Out</Button>
{/if}

<slot />
