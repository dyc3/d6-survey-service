ALTER TABLE surveys
	ADD COLUMN created_at TIMESTAMP,
	ADD COLUMN updated_at TIMESTAMP;
UPDATE surveys
	SET created_at = now(),
		updated_at = now();
ALTER TABLE surveys
	ALTER COLUMN created_at SET NOT NULL,
	ALTER COLUMN updated_at SET NOT NULL,
	ALTER COLUMN created_at SET DEFAULT now(),
	ALTER COLUMN updated_at SET DEFAULT now();
