use std::collections::HashMap;

use serde::Serialize;

use super::ConnectAction;

macro_rules! r {
    ($s:ty, $action:expr, $o:ty) => {
        impl ConnectAction for $s {
            type Output = $o;
            fn action() -> &'static str {
                $action
            }
        }
    };
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note<'a> {
    pub deck_name: &'a str,
    pub model_name: &'a str,
    pub fields: &'a HashMap<&'a str, &'a str>,
    pub tags: &'a Vec<&'a str>,
}

#[derive(Serialize)]
pub struct AddNote<'a> {
    pub note: &'a Note<'a>,
}
r!(AddNote<'_>, "addNote", i64);

#[derive(Serialize)]
pub struct CanAddNotes<'a> {
    pub notes: &'a Vec<&'a Note<'a>>,
}
r!(CanAddNotes<'_>, "canAddNotes", Vec<bool>);

#[derive(Serialize)]
pub struct FindNotes<'a> {
    pub query: &'a str,
}
r!(FindNotes<'_>, "findNotes", Vec<i64>);

#[derive(Serialize)]
pub struct GuiBrowse<'a> {
    pub query: &'a str,
}
r!(GuiBrowse<'_>, "guiBrowse", Vec<i64>);

#[derive(Serialize)]
pub struct DeckNames {}
r!(DeckNames, "deckNames", Vec<String>);

#[derive(Serialize)]
pub struct ModelNames {}
r!(ModelNames, "modelNames", Vec<String>);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelFieldNames<'a> {
    pub model_name: &'a str,
}
r!(ModelFieldNames<'_>, "modelFieldNames", Vec<String>);
