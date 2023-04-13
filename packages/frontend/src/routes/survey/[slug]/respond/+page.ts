import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getSurvey, getSurveyResponse } from '$lib/api';
import type { SurveyResponses } from '$lib/common';

export const load = (async ({ params, fetch, url }) => {
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

	let surveyResponse: SurveyResponses = {};
	if (url.searchParams.has('responder')) {
		const responder = url.searchParams.get('responder');
		if (!responder) throw error(400, 'responder query param must be non-empty string if it exists');
		const response = await getSurveyResponse(surveyId, responder, { fetch });
		if (!response.ok) {
			// TODO: make status codes accessible instead?
			if (response.error.message === 'NotFound') {
				throw error(404, 'Survey Response not found');
			} else if (response.error.message === 'NotPublished') {
				throw error(403, 'Survey not published');
			} else {
				throw error(500, 'Internal server error');
			}
		}
		surveyResponse = response.value.content;
	}

	return {
		slug: params.slug,
		surveyId,
		survey: response.value,
		surveyResponse
	};
}) satisfies PageLoad;
