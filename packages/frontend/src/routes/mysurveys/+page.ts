import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getSurveyList } from '$lib/api';

export const ssr = false;

export const load = (async () => {
	const response = await getSurveyList();

	if (!response.ok) {
		throw error(500, 'Internal server error');
	}

	return {
		surveys: response.value
	};
}) satisfies PageLoad;
