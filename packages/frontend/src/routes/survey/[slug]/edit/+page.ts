import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { getSurveyAuth } from '$lib/api';

export const load = (async ({ params }) => {
	const surveyId = parseInt(params.slug);
	return {
		slug: params.slug,
		surveyId,
	};

}) satisfies PageLoad;