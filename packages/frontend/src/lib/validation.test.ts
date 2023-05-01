import { describe, it, expect } from 'vitest';
import { isValidationError } from '$lib/api';
import type { ApiErrorResponse, ValidationError } from '$lib/common';
import { buildErrorMapFromFields, buildErrorMapFromUuids } from './validation';

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

describe("validation error aggregators", () => {
	it("should collect errors by field name", () => {
		const errors: ValidationError[] = [
			{
				type: "BadValue",
				data: {
					field: "foo",
					message: "invalid",
				}
			},
			{
				type: "Required",
				data: {
					field: "bar",
				}
			},
		];

		const result = buildErrorMapFromFields(errors);

		expect(result.size).toEqual(2);
		expect(result.get("foo")).toEqual([errors[0]]);
		expect(result.get("bar")).toEqual([errors[1]]);
	});

	it("should collect multiple errors by field name", () => {
		const errors: ValidationError[] = [
			{
				type: "BadValue",
				data: {
					field: "foo",
					message: "invalid",
				}
			},
			{
				type: "Required",
				data: {
					field: "foo",
				}
			},
		];

		const result = buildErrorMapFromFields(errors);

		expect(result.size).toEqual(1);
		expect(result.get("foo")).toEqual(errors);
	});

	it("should collect errors by item UUID", () => {
		const errors: ValidationError[] = [
			{
				type: "Inner",
				data: {
					field: "foo",
					uuid: "123",
					inner: {
						type: "Required",
						data: {
							field: "bar",
						}
					}
				}
			},
			{
				type: "Inner",
				data: {
					field: "foo",
					uuid: "456",
					inner: {
						type: "Required",
						data: {
							field: "bar",
						}
					}
				}
			},
		];

		const result = buildErrorMapFromUuids(errors);

		expect(result.size).toEqual(2);
		// @ts-expect-error - we know inner exists, ignoring TS error for test brevity
		expect(result.get("123")).toEqual([errors[0].data.inner]);
		// @ts-expect-error - we know inner exists, ignoring TS error for test brevity
		expect(result.get("456")).toEqual([errors[1].data.inner]);
	})
})
