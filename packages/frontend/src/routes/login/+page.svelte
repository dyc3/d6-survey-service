<script lang="ts">
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import { loginUser, registerUser } from '$lib/api';
	import { jwt } from '../../stores';
	import { goto } from '$app/navigation';
	import type { UserToken } from '$lib/common';
	import type { ApiResponse } from '$lib/api';
	import ButtonGroup from '$lib/ui/ButtonGroup.svelte';
	import Panel from '$lib/ui/Panel.svelte';

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

<Panel border>
	<div class="flex-center" style="width:fit-content; flex-direction: column; margin: auto">
		<ButtonGroup
			orientation="horizontal"
			buttons={['Log In', 'Register']}
			forceSelection={true}
			bind:selected
			role="tab"
		/>
		{#if tab === 0}
			<div class="info-container">
				<TextBox name="username" placeholder="Username" bind:value={username} /> <br />
				<TextBox name="password" password placeholder="Password" bind:value={password} /> <br />
				<Button --margin="5px" type="submit" kind="primary" on:click={doLogin}>Submit</Button>
				<span>{response}</span>
			</div>
		{:else if tab === 1}
			<div class="info-container">
				<TextBox name="username" placeholder="New Username" bind:value={username} /> <br />
				<TextBox name="password" password placeholder="New Password" bind:value={password} /> <br />
				<Button --margin="5px" type="submit" kind="primary" on:click={doRegister}>Submit</Button>
				<span>{response}</span>
			</div>
		{/if}
	</div>
</Panel>

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
