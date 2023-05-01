import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function createJWTStore() {
	const { set: innerset } = writable<string | undefined>(undefined);
	const loggedIn = writable<boolean>(false);

	if (browser) {
		const token = localStorage.getItem('token');
		if (token !== null) {
			loggedIn.set(true);
		}
	}

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
		loggedIn.set(true);
	}

	function logout() {
		set(undefined);
		loggedIn.set(false);
	}

	return {
		get,
		login,
		logout,
		loggedIn,
	};
}

export const jwt = createJWTStore();
