/**
 * Provides a type-checked API for interacting with the backend.
 *
 * Basically just a thin wrapper around fetch() that adds type checking.
 */

import type {
	Survey,
	ApiErrorResponse,
	SurveyPatch,
	UserLoginParams,
	UserToken,
	ValidationError
} from './common';
import { jwt } from '../stores';
import { browser } from '$app/environment';

const API_URL = 'http://localhost:5347';

export type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
export type ApiResponse<T> = Result<T, ApiErrorResponse<any>>;
export type ExtraOptions = { fetch?: typeof fetch; token?: string };

type ApiRequestOptions = RequestInit & ExtraOptions;

function isValidationError(
	apiErr: ApiErrorResponse<any>
): apiErr is ApiErrorResponse<ValidationError> {
	if (!apiErr.message) return false;
	if (typeof apiErr.message !== 'string') return false;
	return (
		typeof apiErr.message !== 'object' &&
		Object.prototype.hasOwnProperty.call(apiErr.message, 'ValidationError')
	);
}

async function apiReq<T>(path: string, options?: ApiRequestOptions): Promise<ApiResponse<T>> {
	const realfetch = options?.fetch ?? fetch;
	const response = await realfetch(`${API_URL}${path}`, options);

	let apiResponse: ApiResponse<T>;
	if (response.ok) {
		if (response.headers.get('Content-Type')?.startsWith('application/json')) {
			apiResponse = { ok: true, value: await response.json() };
		} else {
			apiResponse = { ok: true, value: {} as T };
		}
	} else {
		apiResponse = { ok: false, error: await response.json() };
	}
	return apiResponse;
}

async function apiReqAuth<T>(path: string, options?: ApiRequestOptions): Promise<ApiResponse<T>> {
	const token = options?.token ?? (browser ? jwt.get() : undefined);
	if (!token) {
		if (!browser) {
			throw new Error(
				"Can't make authenticated request from server unless token is provided, see #42"
			);
		}
		throw new Error(`Not logged in, cannot make authenticated request to ${path}`);
	}
	return apiReq(path, {
		...options,
		headers: {
			...options?.headers,
			Authorization: `Bearer ${token}`
		}
	});
}

export async function loginUser(
	params: UserLoginParams,
	opts?: ExtraOptions
): Promise<ApiResponse<UserToken>> {
	return apiReq(`/api/user/login`, {
		method: 'POST',
		body: JSON.stringify(params),
		...opts
	});
}

export async function registerUser(
	params: UserLoginParams,
	opts?: ExtraOptions
): Promise<ApiResponse<UserToken>> {
	return apiReq(`/api/user/register`, {
		method: 'POST',
		body: JSON.stringify(params),
		...opts
	});
}

export async function getSurvey(
	survey_id: number,
	opts?: ExtraOptions
): Promise<ApiResponse<Survey>> {
	return apiReq(`/api/survey/${survey_id}`, { ...opts });
}

export async function getSurveyAuth(
	survey_id: number,
	opts?: ExtraOptions
): Promise<ApiResponse<Survey>> {
	return apiReqAuth(`/api/survey/${survey_id}`, { ...opts });
}

export async function createSurvey(opts?: ExtraOptions): Promise<ApiResponse<Survey>> {
	return apiReqAuth(`/api/survey/create`, { method: 'POST', ...opts });
}

export async function editSurvey(
	survey_id: number,
	survey: SurveyPatch,
	opts?: ExtraOptions
): Promise<ApiResponse<Survey>> {
	return apiReqAuth(`/api/survey/${survey_id}`, {
		method: 'PATCH',
		body: JSON.stringify(survey),
		...opts
	});
}
