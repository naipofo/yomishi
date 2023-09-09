use rusqlite::{params, Connection};

use crate::dict::parser::kanji::Kanji;

pub fn insert_kanjis_bulk(
    conn: &Connection,
    terms: Vec<Kanji>,
    dictionary_id: i64,
) -> rusqlite::Result<()> {
    let mut prep = conn.prepare_cached(
        "INSERT INTO kanjis(
            character,
            onyomi,
            kunyomi,
            kanji_tags,
            meaning,
            various,
            dictionary
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )?;

    terms
        .iter()
        .map(|t| {
            prep.insert(params![
                t.character,
                t.onyomi,
                t.kunyomi,
                serde_json::to_string(&t.kanji_tags).unwrap(),
                serde_json::to_string(&t.meaning).unwrap(),
                serde_json::to_string(&t.various).unwrap(),
                dictionary_id
            ])
            .map(|_| ())
        })
        .collect::<rusqlite::Result<_>>()?;
    prep.discard();
    Ok(())
}
