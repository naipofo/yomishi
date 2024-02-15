use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::dict::DictIndex;

use super::Database;

#[derive(Debug, Deserialize)]
struct DictId {
    id: Thing,
}

impl Database {
    pub async fn insert_dictionary(
        &self,
        DictIndex {
            title, revision, ..
        }: &DictIndex,
    ) -> surrealdb::Result<String> {
        #[derive(Debug, Serialize)]
        struct InsertDict<'a> {
            title: &'a str,
            revision: &'a str,
        }
        let mut created: Vec<DictId> = self
            .conn
            .create("dictionary")
            .content(InsertDict { title, revision })
            .await?;
        Ok(created.remove(0).id.id.to_string())
    }

    pub async fn dict_exists(
        &self,
        DictIndex {
            title, revision, ..
        }: &DictIndex,
    ) -> surrealdb::Result<bool> {
        Ok(!self
            .conn
            .query("SELECT * FROM dictionary WHERE title = $title AND revision = $revision")
            .bind(("title", title))
            .bind(("revision", revision))
            .await?
            .take::<Vec<DictId>>(0)?
            .is_empty())
    }

    pub async fn get_dict_by_id(&self, id: &str) -> surrealdb::Result<String> {
        #[derive(Debug, Deserialize)]
        struct DictName {
            title: String,
        }

        Ok(self
            .conn
            .query("SELECT title FROM $d")
            .bind((
                "d",
                Thing {
                    tb: "dictionary".to_owned(),
                    id: id.into(),
                },
            ))
            .await?
            .take::<Vec<DictName>>(0)?
            .remove(0)
            .title)
    }

    pub async fn get_dicts(&self) -> surrealdb::Result<Vec<(String, DictIndex)>> {
        #[derive(Debug, Deserialize)]
        struct DictData {
            id: String,
            revision: String,
            title: String,
        }
        let data: Vec<DictData> = self.conn.select("dictionary").await?;
        Ok(data
            .into_iter()
            .map(
                |DictData {
                     id,
                     revision,
                     title,
                 }| {
                    (
                        id,
                        DictIndex {
                            title,
                            revision,
                            format: 3,
                        },
                    )
                },
            )
            .collect())
    }
}
