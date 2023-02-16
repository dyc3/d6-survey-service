import type { PageLoad } from './$types';
import type { Survey } from '$lib/common';

export const load = (async ({ params }) => {
	// TODO: don't hardcode api host
	// TODO: handle errors returned by the API
	const survey: Survey = await fetch(`http://localhost:5173/api/survey/${params.slug}`).then((r) =>
		r.json()
	);

	return {
		slug: params.slug,
		survey
	};
}) satisfies PageLoad;
