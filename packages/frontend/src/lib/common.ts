/*
 Generated by typeshare 1.0.0
*/

export type SurveyQuestions = SurveyQuestion[];

export interface ApiErrorResponse<R> {
	message: R;
}

export interface Survey {
	id: number;
	title: string;
	description: string;
	published: boolean;
	owner_id: number;
	questions: SurveyQuestions;
}

/** Represents a partial update to a survey */
export interface SurveyPatch {
	title?: string;
	description?: string;
	published?: boolean;
	questions?: SurveyQuestions;
}

/** Used to list surveys, like on the page where you can see all your surveys */
export interface ListedSurvey {
	id: number;
	title: string;
	description: string;
	published: boolean;
	owner_id: number;
}

export interface SurveyQuestion {
	uuid: string;
	required: boolean;
	question: Question;
}

export interface QText {
	prompt: string;
	description: string;
	multiline: boolean;
}

/** Represents a question like \"On a scale of 1 to N, how do you feel about X?\" */
export interface QRating {
	prompt: string;
	description: string;
	max_rating: number;
}

export interface QMultipleChoice {
	prompt: string;
	description: string;
}

export interface SurveyResponse {
	survey_id: number;
	responder: string;
	content: Record<string, Response>;
}

export interface RText {
	text: string;
}

export interface RRating {
	rating: number;
}

export interface RMultipleChoice {
	selected: string[];
}

export interface UserLoginParams {
	username: string;
	password: string;
}

export interface UserToken {
	token: string;
}

export type Question =
	| { type: 'Text'; content: QText }
	| { type: 'Rating'; content: QRating }
	| { type: 'MultipleChoice'; content: QMultipleChoice };
