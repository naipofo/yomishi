use crate::dict::parser::term::Term;

use super::Database;

use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Result};

impl Database {
    pub async fn insert_terms_bulk(&self, terms: Vec<Term>, dictionary_id: &str) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct InsertTerm<'a> {
            definition_tags: String,
            dictionary: Thing,
            expression: &'a str,
            glossary: String,
            reading: &'a str,
            rules: &'a str,
            score: &'a i64,
            sequence: &'a i64,
            tags: Vec<Thing>,
        }
        self.conn
            .query("INSERT INTO term $terms")
            .bind((
                "terms",
                terms
                    .iter()
                    .map(
                        |Term {
                             expression,
                             reading,
                             definition_tags,
                             rules,
                             score,
                             glossary,
                             sequence,
                             term_tags,
                         }| InsertTerm {
                            dictionary: Thing {
                                tb: "dictionary".to_owned(),
                                id: dictionary_id.into(),
                            },
                            expression,
                            reading,
                            definition_tags: serde_json::to_string(&definition_tags).unwrap(),
                            rules,
                            score,
                            sequence,
                            glossary: serde_json::to_string(&glossary).unwrap(),
                            tags: term_tags
                                .iter()
                                .map(|t| Thing {
                                    tb: "tag".to_string(),
                                    id: surrealdb::sql::Id::String(t.to_string()),
                                })
                                .collect(),
                        },
                    )
                    .collect::<Vec<_>>(),
            ))
            .await?;

        Ok(())
    }
    pub async fn get_terms(&self, term: &str) -> Result<Vec<(Term, String)>> {
        #[derive(Debug, Deserialize)]
        struct TermData {
            definition_tags: String,
            dictionary: Thing,
            expression: String,
            glossary: String,
            reading: String,
            rules: String,
            score: i64,
            sequence: i64,
            // TODO: Query tags together with terms
            tags: Vec<Thing>,
        }

        let data: Vec<TermData> = self
            .conn
            .query("SELECT * FROM term WHERE expression = $expression")
            .bind(("expression", term))
            .await?
            .take(0)?;

        Ok(data
            .into_iter()
            .map(
                |TermData {
                     definition_tags,
                     dictionary,
                     expression,
                     glossary,
                     reading,
                     rules,
                     score,
                     sequence,
                     tags,
                 }| {
                    (
                        Term {
                            expression,
                            reading,
                            definition_tags: serde_json::from_str(&definition_tags).unwrap(),
                            rules,
                            score,
                            glossary: serde_json::from_str(&glossary).unwrap(),
                            sequence,
                            term_tags: tags.into_iter().map(|t| t.id.to_string()).collect(),
                        },
                        dictionary.id.to_string(),
                    )
                },
            )
            .collect())
    }
}
