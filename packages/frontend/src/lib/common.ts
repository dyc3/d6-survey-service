/*
 Generated by typeshare 1.5.0
*/

export type Question =
	| { type: 'Text'; content: QText }
	| { type: 'Rating'; content: QRating }
	| { type: 'MultipleChoice'; content: QMultipleChoice };

export interface SurveyQuestion {
	uuid: string;
	required: boolean;
	question: Question;
}

export type SurveyQuestions = SurveyQuestion[];

export type Response =
	| { type: 'Text'; content: RText }
	| { type: 'Rating'; content: RRating }
	| { type: 'MultipleChoice'; content: RMultipleChoice };

export type SurveyResponses = Record<string, Response>;

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
	created_at: string;
	updated_at: string;
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

export interface SurveyResponse {
	survey_id: number;
	responder_uuid: string;
	content: SurveyResponses;
	created_at: string;
	updated_at: string;
}

export interface QText {
	prompt: string;
	description: string;
	multiline: boolean;
}

/** Represents a question like "On a scale of 1 to N, how do you feel about X?" */
export interface QRating {
	prompt: string;
	description: string;
	max_rating: number;
	min_text: string;
	max_text: string;
}

export interface Choice {
	uuid: string;
	text: string;
}

export interface QMultipleChoice {
	prompt: string;
	description: string;
	multiple: boolean;
	choices: Choice[];
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

export interface ResponseAccepted {
	responder_uuid: string;
}

export interface UserLoginParams {
	username: string;
	password: string;
}

export interface UserToken {
	token: string;
}

export type ValidationError =
	| {
			type: 'Required';
			data: {
				field: string;
			};
	  }
	| {
			type: 'NotInRange';
			data: {
				field: string;
				value: number;
				min: number;
				max: number;
			};
	  }
	| {
			type: 'NotUnique';
			data: {
				field: string;
				value: string;
			};
	  }
	| {
			type: 'NotFound';
			data: {
				field: string;
				uuid: string;
			};
	  }
	| {
			type: 'MismatchedTypes';
			data: {
				uuid: string;
			};
	  }
	| {
			type: 'BadValue';
			data: {
				field: string;
				message: string;
			};
	  }
	| {
			type: 'Inner';
			data: {
				/** The name of the field that failed validation. */
				field: string;
				/** The UUID of the object inside the field that failed validation. */
				uuid: string;
				inner: ValidationError;
			};
	  };
