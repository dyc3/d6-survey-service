import type { PageServerLoad } from './$types';
import { getSurveyAuth } from '$lib/api';

export const load = (async ({ cookies }) => {
	const token = cookies.get('token');

	return {
		survey: await getSurveyAuth(2, { fetch, token })
	};
}) satisfies PageServerLoad;