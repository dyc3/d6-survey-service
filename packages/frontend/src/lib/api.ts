/**
 * Provides a type-checked API for interacting with the backend.
 *
 * Basically just a thin wrapper around fetch() that adds type checking.
 */

import type { Survey, ApiErrorResponse, SurveyPatch, UserLoginParams, UserToken } from './common';
import { jwt } from '../stores';

const API_URL = 'http://localhost:5347'; // TODO: see #42

export type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
export type ApiResponse<T> = Result<T, ApiErrorResponse<any>>;

async function apiReq<T>(path: string, options?: RequestInit | undefined): Promise<ApiResponse<T>> {
	const response = await fetch(`${API_URL}${path}`, options);

	let apiResponse: ApiResponse<T>;
	if (response.ok) {
		apiResponse = { ok: true, value: await response.json() };
	} else {
		apiResponse = { ok: false, error: await response.json() };
	}
	return apiResponse;
}

async function apiReqAuth<T>(
	path: string,
	options?: RequestInit | undefined
): Promise<ApiResponse<T>> {
	const token = jwt.get();
	if (!token) {
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

export async function loginUser(params: UserLoginParams): Promise<ApiResponse<UserToken>> {
	return apiReq(`/api/user/login`, {
		method: 'POST',
		body: JSON.stringify(params)
	});
}

export async function registerUser(params: UserLoginParams): Promise<ApiResponse<UserToken>> {
	return apiReq(`/api/user/register`, {
		method: 'POST',
		body: JSON.stringify(params)
	});
}

export async function getSurvey(survey_id: number): Promise<ApiResponse<Survey>> {
	return apiReq(`/api/survey/${survey_id}`);
}

export async function createSurvey(): Promise<ApiResponse<Survey>> {
	return apiReqAuth(`/api/survey/create`, { method: 'POST' });
}

export async function editSurvey(
	survey_id: number,
	survey: SurveyPatch
): Promise<ApiResponse<Survey>> {
	return apiReqAuth(`/api/survey/${survey_id}`, {
		method: 'PATCH',
		body: JSON.stringify(survey)
	});
}
