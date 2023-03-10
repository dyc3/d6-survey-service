import { getSurveyAuth } from '$lib/api';
import type { PageLoad } from './$types';

export const ssr = false;

export const load = (async ({ fetch, }) => {
	return {
		survey: await getSurveyAuth(2, { fetch })
	};
}) satisfies PageLoad;