use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{Id, Thing},
    Result,
};

use crate::dict::parser::tag::Tag;

use super::Database;

impl Database {
    pub async fn insert_tags_bulk(&self, tags: Vec<Tag>, dictionary_id: &str) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct InsertTag<'a> {
            id: &'a str,
            category: &'a str,
            dictionary: Thing,
            notes: &'a str,
            popularity: &'a i64,
            sorting: &'a i64,
        }
        self.conn
            .query("INSERT INTO tag $tags")
            .bind((
                "tags",
                tags.iter()
                    .map(
                        |Tag {
                             name,
                             category,
                             sorting,
                             notes,
                             popularity,
                         }| InsertTag {
                            id: name,
                            category,
                            notes,
                            sorting,
                            popularity,
                            dictionary: Thing::from(("dictionary", Id::from(dictionary_id))),
                        },
                    )
                    .collect::<Vec<_>>(),
            ))
            .await?;

        Ok(())
    }
    pub async fn get_tag_list(&self, names: &[String], _dict_id: &str) -> Result<Vec<Tag>> {
        #[derive(Debug, Deserialize)]
        struct TagData {
            id: String,
            category: String,
            notes: String,
            popularity: i64,
            sorting: i64,
        }

        let data: Vec<TagData> = self
            .conn
            .query("SELECT * FROM tag WHERE name IN $name")
            .bind((
                "name",
                names
                    .iter()
                    .map(|n| Thing::from(("tag", Id::from(n))))
                    .collect::<Vec<_>>(),
            ))
            .await?
            .take(0)?;

        Ok(data
            .into_iter()
            .map(
                |TagData {
                     id,
                     category,
                     notes,
                     popularity,
                     sorting,
                 }| Tag {
                    name: id,
                    category,
                    sorting,
                    notes,
                    popularity,
                },
            )
            .collect())
    }
}
