<script lang="ts">
	import type { ApiResponse } from '$lib/api';
	import type { Survey } from '$lib/common';
	import type { PageData } from './$types';

	export let data: PageData;

	const survey: ApiResponse<Survey> = data.survey;
</script>

{JSON.stringify(survey)}
