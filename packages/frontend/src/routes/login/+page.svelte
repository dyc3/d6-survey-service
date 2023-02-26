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
</script>

<Main>
	<TabGroup>
		Welcome to the Survey App!
		<TabList class='TabContainer' style="background: red;">
			<Tab>Log In</Tab>
			<Tab>Register</Tab>
		</TabList>
		<TabPanels>
			<TabPanel style="width: max-content; margin: auto">
				Log in <br />
				<TextBox placeholder="Username" bind:value={username} /> <br />
				<TextBox placeholder="Password" bind:value={password} /> <br />
				<Button kind="primary" on:click={doLogin}>Submit</Button>
				<span>{response}</span>
			</TabPanel>
			<TabPanel style="width: max-content; margin: auto">
				Create a New User <br />
				<TextBox placeholder="New Username" bind:value={username} /> <br />
				<TextBox placeholder="New Password" bind:value={password} /> <br />
				<Button kind="primary" on:click={doRegister}>Submit</Button>
				<span>{response}</span>
			</TabPanel>
		</TabPanels>
	</TabGroup>
</Main>

<style lang="scss">
	@import '../../lib/ui/main.scss';

	.TabContainer{
		display: flex;
		flex-direction: row;
		background:red;
	}
</style>