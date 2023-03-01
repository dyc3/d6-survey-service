CREATE TABLE responses (
	survey_id SERIAL NOT NULL REFERENCES surveys (id) ON DELETE CASCADE,
	responder_uuid UUID PRIMARY KEY,
	content JSONB NOT NULL DEFAULT '{}'::JSONB,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	UNIQUE(survey_id, responder_uuid)
);

SELECT diesel_manage_updated_at('responses');