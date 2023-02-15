<script>
	import Box from './Box.svelte';

	let name = 'tester';
	let user = { logIn: true };

	function toggle() {
		user.logIn = !user.logIn;
	}
</script>

<main>
	<Box>
		<h2>Welcome back {name}!</h2>
		<!---- buttons ---->
		<div class="loginOptions">
			<button on:click={toggle}>Login</button>
			<button on:click={toggle}>Register</button>
		</div>
		<!-- buttons that will toggle on/off login/register pages -->
		{#if user.logIn}
			<p>Log in menu here</p>
		{:else}
			<p>Registration menu here</p>
		{/if}
	</Box>
	<Box />
</main>

<style>
	main {
		align-items: center;
		text-align: center;
	}
	.loginOptions {
		/* border: 1px; */
		/* border: 1px solid #4f4f4f; */
		padding:0px;
		display:inline-block;
		/* border-radius: 8px; */
	}

	button {
		margin: 0px;
		padding: 1em;
		border: none;
		background-color: white;
		border: 1px;
		border: 1px solid #4f4f4f;
		border-radius: 8px;
	}

	button:focus {
		background-color: #4f4f4f;
		color: white;
	}

	/* .disabled {
		opacity: 0.6;
		cursor: not-allowed;
	} */

</style>
