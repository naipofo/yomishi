use crate::dict::parser::{tag::Tag, term::Term};

use super::Database;

use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{Id, Thing},
    Result,
};

#[derive(Debug, Deserialize)]
pub struct LookupData {
    pub dictionary: Thing,
    pub dictionary_name: String,
    pub expression: String,
    pub reading: String,
    pub glossary: String,
    pub rules: String,
    pub tags: Vec<LookupDataTag>,
    pub definition_tags: Vec<LookupDataTag>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct LookupDataTag {
    pub id: Thing,
    pub dictionary: Thing,
    pub category: String,
    pub notes: String,
}

impl LookupDataTag {
    pub fn into_model(self) -> Tag {
        let LookupDataTag {
            id,
            category,
            notes,
            ..
        } = self;
        Tag {
            name: id.id.to_string(),
            category,
            notes,
            sorting: 0,
            popularity: 0,
        }
    }
}

impl Database {
    pub async fn insert_terms_bulk(&self, terms: Vec<Term>, dictionary_id: &str) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct InsertTerm<'a> {
            definition_tags: Vec<Thing>,
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
                            dictionary: ("dictionary", dictionary_id).into(),
                            expression,
                            reading,
                            definition_tags: tags_to_things(definition_tags),
                            rules,
                            score,
                            sequence,
                            glossary: serde_json::to_string(&glossary).unwrap(),
                            tags: tags_to_things(term_tags),
                        },
                    )
                    .collect::<Vec<_>>(),
            ))
            .await?;

        Ok(())
    }

    pub async fn new_term_lookup(
        &self,
        terms: Vec<&str>,
        disabled_dicts: &[String],
    ) -> Result<Vec<LookupData>> {
        let disabled_dicts = disabled_dicts
            .iter()
            .map(|e| Thing::from(("dictionary", Id::from(e))))
            .collect::<Vec<_>>();

        self.conn
            .query(include_str!("terms/lookup.surql"))
            .bind(("terms", &terms))
            .bind(("disabled_dicts", disabled_dicts))
            .await?
            .take(0)
    }

    pub async fn get_terms(&self, term: &str) -> Result<Vec<(Term, String)>> {
        #[derive(Debug, Deserialize)]
        struct TermData {
            dictionary: Thing,
            expression: String,
            glossary: String,
            reading: String,
            rules: String,
            score: i64,
            sequence: i64,
            // TODO: Query tags together with terms
            definition_tags: Vec<Thing>,
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
                            definition_tags: definition_tags
                                .into_iter()
                                .map(|t| t.id.to_string())
                                .collect(),
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

fn tags_to_things(term_tags: &[String]) -> Vec<Thing> {
    term_tags
        .iter()
        .map(|t| Thing::from(("tag", Id::from(t))))
        .collect()
}
