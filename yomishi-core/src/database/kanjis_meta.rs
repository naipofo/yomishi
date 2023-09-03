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

    terms
        .iter()
        .map(|t| {
            prep.insert(params![
                t.kanji,
                serde_json::to_string(&t.value).unwrap(),
                dictionary_id
            ])
            .map(|_| ())
        })
        .collect::<rusqlite::Result<_>>()?;
    prep.discard();
    Ok(())
}
