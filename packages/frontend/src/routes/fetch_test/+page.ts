import { getSurveyAuth } from '$lib/api';
import type { PageLoad } from './$types';
import { jwt } from '../../stores';

export const ssr = false;

export const load = (async ({ fetch, data }) => {
	const token = jwt.get();
	console.log("token", token)
	return {
		survey: await getSurveyAuth(2, { fetch, token })
	};
}) satisfies PageLoad;