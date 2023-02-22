<script>
	import { Tab, TabGroup, TabList, TabPanel, TabPanels } from '@rgossiaux/svelte-headlessui';
	import Button from '$lib/ui/Button.svelte';
	import TextBox from '$lib/ui/TextBox.svelte';
</script>

<div style="justify-content: center;">
<TabGroup>
	<TabList>
		<Tab><Button>Log In</Button></Tab>
		<Tab><Button>Register</Button></Tab>
	</TabList>
	<TabPanels>
		<TabPanel>
				Content 1 test <br>
				<TextBox placeholder="Username" /> <br>
				<TextBox placeholder="Password" /> <br>
				<Button>Submit</Button> 
		</TabPanel>
		<TabPanel>Content 2</TabPanel>
	</TabPanels>
</TabGroup>

</div>