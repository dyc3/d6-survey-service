/*
 Generated by typeshare 1.0.0
*/

export interface Survey {
	id: number;
	title: string;
	description: string;
	published: boolean;
	owner_id: number;
	questions: SurveyQuestion[];
}

export interface SurveyQuestion {
	uuid: Uuid;
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

export interface UserLoginParams {
	username: string;
	password: string;
}

export interface UserToken {
	token: string;
}

export type Question = { type: 'Text'; content: QText } | { type: 'Rating'; content: QRating };
