<script lang="ts">
	import Main from '$lib/ui/Main.svelte';
	import { Tab, TabGroup, TabList, TabPanel, TabPanels } from '@rgossiaux/svelte-headlessui';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import { loginUser, registerUser } from '$lib/api';
	import { jwt } from '../../stores';

	let username = '';
	let password = '';
	let response = '';

	/** @type {import('./$types').LayoutServerLoad} */
	// git commit to update
	async function doLogin() {
		let resp = await loginUser({ username, password });
		console.log('hi');
		if (resp.ok) {
			// save token into jwt in stores.ts if successful
			jwt.login(resp.value.token);
			try {
				window.location.href = '/mysurveys';
			} catch (error) {
				console.log(error);
			}
		}
	}

	async function doRegister() {
		try {
			let resp = await registerUser({ username, password });
			if (resp.ok) {
				jwt.login(resp.value.token);
			}
		} catch (error) {
			console.log(error);
			response = 'Error while registering.';
		}
	}

</script>

<Main>
	<TabGroup>
		Welcome to the Survey App!
		<TabList>
			<Tab style="background-color: white; border: 0px;"><Button toggleable={true}>Log In</Button></Tab>
			<Tab style="background-color: white; border: 0px;"><Button toggleable={true}>Register</Button></Tab>
		</TabList>
		<TabPanels>
			<TabPanel style="width: max-content; margin: auto">
				Log in <br />
				<TextBox placeholder="Username" bind:value={username} /> <br />
				<TextBox placeholder="Password" bind:value={password} /> <br />
				<Button on:click={doLogin}>Submit</Button>
			</TabPanel>
			<TabPanel style="width: max-content; margin: auto">
				Create a New User <br />
				<TextBox placeholder="New Username" bind:value={username} /> <br />
				<TextBox placeholder="New Password" bind:value={password} /> <br />
				<Button on:click={doRegister}>Submit</Button>
				<span>{response}</span>
			</TabPanel>
		</TabPanels>
	</TabGroup>
</Main>