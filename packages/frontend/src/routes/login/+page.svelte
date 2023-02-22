<script lang="ts">
	import Main from '$lib/ui/Main.svelte';
	import { Tab, TabGroup, TabList, TabPanel, TabPanels } from '@rgossiaux/svelte-headlessui';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
	import {loginUser, registerUser} from '$lib/api';
	import {jwt} from '../../stores';

	let response ='';
	let username = '';
	let password = '';

	async function doLogin() {
		// cases for invalid inputs
		let resp = await loginUser({username, password});
		// check if throws error/suceeds or not (check returns)
		if (resp.ok) {
			// save token into jwt in stores.ts if successful
			jwt.login(resp.value.token); 
		}
		// later, error handling
	}

	async function doRegister() {
		let resp = await registerUser({username, password});
		console.log(resp);
		if (resp.ok) {
			jwt.login(resp.value.token); 
		}
	}
</script>


Hello

<Main>
	<div style="justify-content: center;">
		<TabGroup>
			<TabList>
				<Tab><Button>Log In</Button></Tab>
				<Tab><Button>Register</Button></Tab>
			</TabList>
			<TabPanels>
				<TabPanel>
						Content 1 test <br>
						<TextBox placeholder="Username" bind:value={username}/> <br>
						<TextBox placeholder="Password" bind:value={password}/> <br>
						<Button on:click={doLogin}>Submit</Button> 
				</TabPanel>
				<TabPanel>
					
					Content 2 test <br>
					<TextBox placeholder="Username" bind:value={username}/> <br>
					<TextBox placeholder="Password" bind:value={password}/> <br>
					<Button on:click={doRegister}>Submit</Button> 
				</TabPanel>
			</TabPanels>
		</TabGroup>
		</div>
</Main>
