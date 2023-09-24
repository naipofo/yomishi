CREATE TABLE IF NOT EXISTS dictionaries(
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    revision TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS terms(
    id INTEGER PRIMARY KEY,
    expression TEXT NOT NULL,
    reading TEXT NOT NULL,
    definition_tags TEXT,
    rules TEXT NOT NULL,
    score INTEGER NOT NULL,
    glossary TEXT NOT NULL,
    sequence INTEGER NOT NULL,
    term_tags TEXT NOT NULL,
    dictionary INTEGER NOT NULL,
    FOREIGN KEY(dictionary) REFERENCES dictionaries(id)
);
CREATE TABLE IF NOT EXISTS terms_meta(
    id INTEGER PRIMARY KEY,
    term TEXT NOT NULL,
    reading text,
    entry TEXT NOT NULL,
    dictionary INTEGER NOT NULL,
    FOREIGN KEY(dictionary) REFERENCES dictionaries(id)
);
CREATE TABLE IF NOT EXISTS kanjis(
    id INTEGER PRIMARY KEY,
    character TEXT NOT NULL,
    onyomi TEXT NOT NULL,
    kunyomi TEXT NOT NULL,
    kanji_tags TEXT NOT NULL,
    meaning TEXT NOT NULL,
    various TEXT NOT NULL,
    dictionary INTEGER NOT NULL,
    FOREIGN KEY(dictionary) REFERENCES dictionaries(id)
);
CREATE TABLE IF NOT EXISTS kanjis_meta(
    id INTEGER PRIMARY KEY,
    kanji TEXT NOT NULL,
    value INTEGER NOT NULL,
    dictionary INTEGER NOT NULL,
    FOREIGN KEY(dictionary) REFERENCES dictionaries(id)
);
CREATE TABLE IF NOT EXISTS tags(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    sorting INTEGER NOT NULL,
    notes TEXT NOT NULL,
    popularity INTEGER NOT NULL,
    dictionary INTEGER NOT NULL,
    FOREIGN KEY(dictionary) REFERENCES dictionaries(id)
);

CREATE TABLE IF NOT EXISTS config(
    id INTEGER PRIMARY KEY,
    proto BLOB NOT NULL
);