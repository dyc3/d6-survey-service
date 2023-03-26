import type { ValidationError } from "./common";

export function buildErrorMapFromFields(errors: ValidationError[]): Map<string, ValidationError[]> {
	let errmap = new Map<string, ValidationError[]>();

	errors.forEach((err) => {
		let prev: ValidationError[] | undefined;
		switch (err.type) {
			case 'BadValue':
				prev = errmap.get(err.data.field);
				if (prev) {
					prev.push(err);
				} else {
					errmap.set(err.data.field, [err]);
				}
				break;
			case 'Required':
				prev = errmap.get(err.data.field);
				if (prev) {
					prev.push(err);
				} else {
					errmap.set(err.data.field, [err]);
				}
				break;
			case 'Inner':
				prev = errmap.get(err.data.field);
				if (prev) {
					prev.push(err);
				} else {
					errmap.set(err.data.field, [err]);
				}
				break;
			default:
				console.warn('Unknown validation error type', err);
				break;
		}
	});

	return errmap;
}

export function buildErrorMapFromUuids(errors: ValidationError[]) {
	let errmap = new Map<string, ValidationError[]>();

	for (const error of errors) {
		if (error.type === 'Inner') {
			let prev = errmap.get(error.data.uuid);
			if (prev) {
				prev.push(error.data.inner);
			} else {
				errmap.set(error.data.uuid, [error.data.inner]);
			}
		} else {
			console.warn('Unhandled error type', error.type);
		}
	}

	return errmap;
}

export function unwrapInnerErrors(errors: ValidationError[]) {
	return errors.map(error => {
		if (error.type === 'Inner') {
			return error.data.inner;
		} else {
			return error;
		}
	});
}