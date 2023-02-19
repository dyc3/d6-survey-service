import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getSurvey } from '$lib/api';

export const load = (async ({ params, fetch }) => {
	const surveyId = parseInt(params.slug);
	const response = await getSurvey(surveyId, { fetch });
	if (!response.ok) {
		// TODO: make status codes accessible instead?
		if (response.error.message === 'NotFound') {
			throw error(404, 'Survey not found');
		} else if (response.error.message === 'NotPublished') {
			throw error(403, 'Survey not published');
		} else {
			throw error(500, 'Internal server error');
		}
	}

	return {
		slug: params.slug,
		surveyId,
		survey: response.value
	};
}) satisfies PageLoad;
