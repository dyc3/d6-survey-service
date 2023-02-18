import { describe, it, expect } from 'vitest';
import { jwt } from './stores';

const localStorageMock = (function () {
	var store: { [key: string]: string | null } = {};

	return {
		getItem(key: string): string | null {
			return store[key] || null;
		},
		setItem(key: string, value: string | null) {
			if (!value) {
				delete store[key];
				return;
			}
			store[key] = value.toString();
		},
		removeItem(key: string) {
			delete store[key];
		},
		clear() {
			store = {};
		}
	};
})();

Object.defineProperty(global, 'localStorage', {
	value: localStorageMock
});

describe('jwt store', () => {
	it('should store token in localStorage', () => {
		jwt.login('foo');

		expect(localStorage.getItem('token')).toEqual('foo');
	});

	it('should remove token on logout', () => {
		localStorage.setItem('token', 'foo');
		jwt.logout();

		expect(localStorage.getItem('token')).toEqual(null);
	});

	it('should get token', () => {
		jwt.login('foo');

		expect(jwt.get()).toEqual('foo');
	});
});
