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
	ValidationError,
	ListedSurvey,
	SurveyResponses,
	ResponseAccepted,
	SurveyResponse
} from './common';
import { jwt } from '../stores';
import { browser } from '$app/environment';

const API_URL = 'http://localhost:5347';

export type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
export type ApiResponse<T> = Result<T, ApiErrorResponse<unknown>>;
export type ExtraOptions = { fetch?: typeof fetch; token?: string; raw?: boolean };

type ApiRequestOptions = RequestInit & ExtraOptions;

export function isValidationError(
	apiErr: ApiErrorResponse<unknown>
): apiErr is ApiErrorResponse<{ ValidationError: ValidationError[] }> {
	if (!apiErr.message) return false;
	if (typeof apiErr.message === 'string') return false;
	return (
		typeof apiErr.message === 'object' &&
		Object.prototype.hasOwnProperty.call(apiErr.message, 'ValidationError')
	);
}

async function apiReq<T>(path: string, options?: ApiRequestOptions): Promise<ApiResponse<T>> {
	const realfetch = options?.fetch ?? fetch;
	const response = await realfetch(`${API_URL}${path}`, options);

	if (options?.raw) {
		if (response.ok) {
			return { ok: true, value: response as unknown as T };
		} else {
			return { ok: false, error: await response.json() };
		}
	}

	let apiResponse: ApiResponse<T>;
	if (response.headers.get('Content-Type')?.startsWith('application/json')) {
		if (response.ok) {
			apiResponse = { ok: true, value: await response.json() };
		} else {
			apiResponse = { ok: false, error: await response.json() };
		}
	} else {
		if (response.ok) {
			apiResponse = { ok: true, value: (await response.text()) as unknown as T };
		} else {
			apiResponse = {
				ok: false,
				error: { message: await response.text() }
			};
		}
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

export async function getSurveyList(opts?: ExtraOptions): Promise<ApiResponse<ListedSurvey[]>> {
	return apiReqAuth(`/api/user/surveys`, { ...opts });
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

export async function deleteSurvey(
	survey_id: number,
	opts?: ExtraOptions
): Promise<ApiResponse<null>> {
	return apiReqAuth(`/api/survey/${survey_id}`, { method: 'DELETE', ...opts });
}

export async function createSurveyResponse(
	survey_id: number,
	responses: SurveyResponses,
	opts?: ExtraOptions
): Promise<ApiResponse<ResponseAccepted>> {
	return apiReq(`/api/survey/${survey_id}/respond`, {
		method: 'POST',
		body: JSON.stringify(responses),
		...opts
	});
}

export async function editSurveyResponse(
	survey_id: number,
	responder: string,
	responses: SurveyResponses,
	opts?: ExtraOptions
): Promise<ApiResponse<null>> {
	return apiReq(`/api/survey/${survey_id}/respond?responder=${responder}`, {
		method: 'PATCH',
		body: JSON.stringify(responses),
		...opts
	});
}

export async function getSurveyResponse(
	survey_id: number,
	responder: string,
	opts?: ExtraOptions
): Promise<ApiResponse<SurveyResponse>> {
	return apiReq(`/api/survey/${survey_id}/respond?responder=${responder}`, {
		method: 'GET',
		...opts
	});
}

interface ExportResponse {
	blob: Blob;
	filename: string;
}

export async function exportResponses(
	survey_id: number,
	opts?: ExtraOptions
): Promise<ApiResponse<ExportResponse>> {
	const resp = await apiReqAuth<Response>(`/api/survey/${survey_id}/export`, {
		raw: true,
		...opts
	});
	if (resp.ok) {
		const filename =
			resp.value.headers.get('content-disposition')?.split('=')[1] ?? 'responses.csv';
		return { ok: true, value: { blob: await resp.value.blob(), filename } };
	}
	return resp;
}
