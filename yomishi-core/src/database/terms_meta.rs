use std::vec;

use serde::Serialize;
use surrealdb::{sql::Thing, Result};

use crate::{dict::parser::term_meta::TermMeta, scan::search::DictionaryTagged};

use super::Database;

impl Database {
    pub async fn insert_terms_meta_bulk(
        &self,
        terms_meta: Vec<TermMeta>,
        dictionary_id: &str,
    ) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct InsertTermMeta<'a> {
            dictionary: Thing,
            entry: String,
            reading: Option<&'a str>,
            term: &'a str,
        }

        self.conn
            .query("INSERT INTO term_meta $terms_meta")
            .bind((
                "terms_meta",
                terms_meta
                    .iter()
                    .map(
                        |TermMeta {
                             term,
                             reading,
                             entry,
                         }| InsertTermMeta {
                            dictionary: Thing {
                                tb: "dictionary".to_owned(),
                                id: dictionary_id.into(),
                            },
                            term,
                            reading: reading.as_deref(),
                            entry: serde_json::to_string(&entry).unwrap(),
                        },
                    )
                    .collect::<Vec<_>>(),
            ))
            .await?;

        Ok(())
    }

    pub async fn get_term_meta(
        &self,
        _term: &str,
        _reading: &str,
    ) -> Result<Vec<DictionaryTagged<TermMeta>>> {
        // TODO: fetching term meta
        Ok(vec![])
    }
}

// impl Database {
//     pub fn get_term_meta(
//         &self,
//         term: &str,
//         reading: &str,
//     ) -> rusqlite::Result<> {
//         let mut prep = self.conn.prepare(
//             "SELECT
//                 term,
//                 reading,
//                 entry,
//                 dictionary
//                 FROM terms_meta
//             WHERE term = ?",
//         )?;
//         let results = prep
//             .query_map(params![term], |e| {
//                 Ok((
//                     TermMeta {
//                         term: e.get(0)?,
//                         reading: e.get(1)?,
//                         entry: serde_json::from_str(&e.get::<_, String>(2)?).unwrap(),
//                     },
//                     e.get(3)?,
//                 ))
//             })?
//             .collect::<rusqlite::Result<Vec<_>>>()?
//             .into_iter()
//             .flat_map(|(e, dict_id)| match &e.reading {
//                 // TODO: nicer filter
//                 Some(r) => {
//                     if *r == reading {
//                         Some(self.get_dict_by_id(&dict_id).map(|d| (dict_id, d, e)))
//                     } else {
//                         None
//                     }
//                 }
//                 None => Some(self.get_dict_by_id(&dict_id).map(|d| (dict_id, d, e))),
//             })
//             .collect::<rusqlite::Result<Vec<_>>>()?
//             .into_iter()
//             .map(|(dictionary_id, dictionary, data)| DictionaryTagged {
//                 dictionary,
//                 dictionary_id,
//                 data,
//             })
//             .collect();

//         Ok(results)
//     }
// }
