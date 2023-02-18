import { writable } from 'svelte/store';

function createJWTStore() {
	const { set: innerset } = writable<string | undefined>(undefined);

	function set(value: string | undefined) {
		innerset(value);
		if (value === undefined) {
			localStorage.removeItem('token');
			return;
		}
		localStorage.setItem('token', value);
	}

	function get(): string | undefined {
		const token = localStorage.getItem('token');
		if (token === null) {
			return undefined;
		}
		return token;
	}

	function login(token: string) {
		set(token);
	}

	function logout() {
		set(undefined);
	}

	return {
		get,
		login,
		logout
	};
}

export const jwt = createJWTStore();
