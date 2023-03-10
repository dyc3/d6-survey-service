<script lang="ts">
	import type { Survey } from '$lib/common';
	import type { PageData } from './$types';

	export let data: PageData;

	const survey: Survey = data.survey;
</script>

{JSON.stringify(survey)}
