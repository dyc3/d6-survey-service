<script lang="ts">
	import Container from '$lib/ui/Container.svelte';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import { loginUser, registerUser } from '$lib/api';
	import { jwt } from '../../stores';
	import { goto } from '$app/navigation';
	import type { UserToken } from '$lib/common';
	import type { ApiResponse } from '$lib/api';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';

	let username = '';
	let password = '';
	let response = '';

	function handleLogin(resp: ApiResponse<UserToken>) {
		if (resp.ok) {
			response = 'Successfully logged in.';
			jwt.login(resp.value.token);
			goto('/mysurveys');
		} else {
			console.error(resp.error);
			response = resp.error.message as string;
		}
	}

	async function doLogin() {
		handleLogin(await loginUser({ username, password }));
	}

	async function doRegister() {
		handleLogin(await registerUser({ username, password }));
	}

	let selected: number[] = [0];
	$: tab = selected[0];
</script>

<Container>
	<ButtonGroup
		orientation="horizontal"
		buttons={['Log In', 'Register']}
		forceSelection={true}
		bind:selected
		role="tab"
	/>
	<!--TODO: Make it so that whichever page is present makes the respective button highlighted.-->
	{#if tab === 0}
		<div class="info-container">
			<TextBox name="username" placeholder="Username" bind:value={username} /> <br />
			<TextBox name="password" placeholder="Password" bind:value={password} /> <br />
			<Button --margin="5px" type="submit" kind="primary" on:click={doLogin}>Submit</Button>
			<span>{response}</span>
		</div>
	{:else if tab === 1}
		<div class="info-container">
			<TextBox name="username" placeholder="New Username" bind:value={username} /> <br />
			<TextBox name="password" placeholder="New Password" bind:value={password} /> <br />
			<Button --margin="5px" type="submit" kind="primary" on:click={doRegister}>Submit</Button>
			<span>{response}</span>
		</div>
	{/if}
</Container>

<style lang="scss">
	@import '../../lib/ui/variables';

	.info-container {
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
		width: 100%;
		padding-top: $main-padding;
	}
</style>
