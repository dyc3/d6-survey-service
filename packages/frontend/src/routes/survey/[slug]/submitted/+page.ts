import type { PageLoad } from './$types';

export const load = (async ({ params, url }) => {
	return {
		slug: params.slug,
		response: url.searchParams.get('responder')
	};
}) satisfies PageLoad;
