import { getSurvey } from '$lib/api';
import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
	return {
		survey: await getSurvey(2, { fetch })
	};
}) satisfies PageLoad;