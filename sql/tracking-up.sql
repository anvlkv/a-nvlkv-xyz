CREATE TABLE tracking (
    id BLOB PRIMARY KEY,
    created_date INTEGER NOT NULL,
    updated_date INTEGER NOT NULL,
    user_agent TEXT NOT NULL,
    inferrence TEXT,
    personal_inquery BLOB,
    wk_download INTEGER
);

ALTER TABLE tracking ADD COLUMN restored_session BLOB REFERENCES tracking(id);
