import { describe, it, expect } from 'vitest';
import { isValidationError } from '$lib/api';
import type { ApiErrorResponse } from '$lib/common';

describe('processing validation errors', () => {
	it('should correctly determine that the response contains validation errors', () => {
		const err: ApiErrorResponse<unknown> = {
			message: {
				ValidationError: []
			}
		};
		expect(isValidationError(err)).toEqual(true);
	});

	it('should correctly determine that the response does NOT contain validation errors', () => {
		let err: ApiErrorResponse<unknown> = {
			message: 'Not found'
		};
		expect(isValidationError(err)).toEqual(false);
		err = {
			message: {
				foo: 'bar'
			}
		};
		expect(isValidationError(err)).toEqual(false);
	});
});
