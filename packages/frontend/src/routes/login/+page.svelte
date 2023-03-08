<script lang="ts">
	import Main from '$lib/ui/Main.svelte';
	import { Tab, TabGroup, TabList, TabPanel, TabPanels } from '@rgossiaux/svelte-headlessui';
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
			response = resp.error.message;
		}
	}

	async function doLogin() {
		handleLogin(await loginUser({ username, password }));
	}

	async function doRegister() {
		handleLogin(await registerUser({ username, password }));
	}

	let group_selected = 0;
</script>


	<div class='signin-container'> 
		<ButtonGroup
		orientation="horizontal"
		buttons={['Login', 'Register']}
		forceSelection={true}
		bind:selected={group_selected}
		/>
		<!--TODO: Make it so that whichever page is present makes the respective button highlighted.-->
		{#if group_selected === 0}
			<div class = 'info-container'>
				<TextBox name="username" placeholder="Username" bind:value={username} /> <br />
				<TextBox name="password" placeholder="Password" bind:value={password} /> <br />
				<Button kind="primary" on:click={doLogin}>Submit</Button>
				<span>{response}</span>
			</div>
		{:else if group_selected === 1}
			<div class = 'info-container'>
				<TextBox name="username" placeholder="New Username" bind:value={username} /> <br />
				<TextBox name="password" placeholder="New Password" bind:value={password} /> <br />
				<Button kind="primary" on:click={doRegister}>Submit</Button>
				<span>{response}</span>
			</div>
		{/if}
	</div>

<style lang="scss">
	@import '../../lib/ui/variables.scss';

	.signin-container{
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
		height: 40vh;
		min-height: fit-content;
		width: 50vw;
		border: 3px solid $color-blue;
		border-radius: 10px;
		margin-left: auto;
		margin-right: auto;
	}

	.info-container{
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
		width: 100%;
	}
</style>