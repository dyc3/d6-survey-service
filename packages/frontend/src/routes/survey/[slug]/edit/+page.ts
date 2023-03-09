import type { PageLoad } from './$types';

export const load = (async ({ params }) => {
	const surveyId = parseInt(params.slug);
	return {
		slug: params.slug,
		surveyId,
	};
}) satisfies PageLoad;