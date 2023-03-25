<script lang="ts">
	import { browser } from '$app/environment';
	import { jwt } from '../stores';
	import { goto } from '$app/navigation';

	if (browser) {
		if (jwt.get() === undefined) {
			goto('/login');
		} else {
			goto('/mysurveys');
		}
	}
</script>
