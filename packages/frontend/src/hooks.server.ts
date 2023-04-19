/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	const response = await resolve(event, {
		filterSerializedResponseHeaders: (name: string, value: string) => true
	});

	return response;
}
