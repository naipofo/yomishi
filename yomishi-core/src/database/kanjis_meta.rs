use rusqlite::{params, Connection};

use crate::dict::parser::kanji_meta::KanjiMeta;

pub fn insert_kanjis_meta_bulk(
    conn: &Connection,
    terms: Vec<KanjiMeta>,
    dictionary_id: i64,
) -> rusqlite::Result<()> {
    let mut prep = conn.prepare_cached(
        "INSERT INTO kanjis_meta(
            kanji,
            value,
            dictionary
        ) VALUES (?, ?, ?)",
    )?;

    terms.iter().try_for_each(|t| {
        prep.insert(params![
            t.kanji,
            serde_json::to_string(&t.value).unwrap(),
            dictionary_id
        ])
        .map(|_| ())
    })?;
    prep.discard();
    Ok(())
}
