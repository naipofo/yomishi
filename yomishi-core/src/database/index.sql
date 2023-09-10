CREATE INDEX IF NOT EXISTS terms_i ON terms (
    expression DESC
);
CREATE INDEX IF NOT EXISTS terms_meta_i ON terms_meta (
    term DESC
);