import { getSurvey } from '$lib/api';
import type { PageLoad } from './$types';

export const load = (async ({ fetch, data }) => {
	return data;
}) satisfies PageLoad;