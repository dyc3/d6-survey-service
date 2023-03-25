import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getSurveyAuth } from '$lib/api';

export const ssr = false;

export const load = (async ({ params }) => {
	const surveyId = parseInt(params.slug);
	const response = await getSurveyAuth(surveyId);

	if (!response.ok) {
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
