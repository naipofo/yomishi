use std::{
    collections::HashMap,
    fs::{read_dir, File},
    path::Path,
};

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use zip::ZipArchive;

use self::parser::{
    kanji::Kanji, kanji_meta::KanjiMeta, parse_bank, tag::Tag, term::Term, term_meta::TermMeta,
    FromBank,
};

pub mod html;
pub mod parser;

#[derive(Debug, Deserialize)]
pub struct DictIndex {
    pub title: String,
    pub revision: String,
    format: i64,
}

pub type LoadedDict = (
    DictIndex,
    Vec<Term>,
    Vec<TermMeta>,
    Vec<Kanji>,
    Vec<KanjiMeta>,
    Vec<Tag>,
);

pub fn import_from_directory<F: FnMut(&DictIndex) -> bool>(
    dir_path: &Path,
    mut should_import: F,
) -> std::io::Result<Vec<LoadedDict>> {
    read_dir(dir_path)?
        .into_iter()
        .filter_map(|e| {
            let mut zip = zip::ZipArchive::new(File::open(e.unwrap().path()).unwrap()).unwrap();
            let index = get_index(&mut zip).unwrap();
            if should_import(&index) {
                Some(import_zip(&mut zip, index))
            } else {
                None
            }
        })
        .collect()
}

pub fn get_index(zip: &mut ZipArchive<File>) -> std::io::Result<DictIndex> {
    let mut ob: HashMap<String, Value> =
        serde_json::from_reader(zip.by_name("index.json")?).unwrap();
    let index = DictIndex {
        title: ob.remove("title").unwrap().to_string(),
        revision: ob.remove("revision").unwrap().to_string(),
        format: ob
            .get("version")
            .unwrap_or(ob.get("format").unwrap())
            .as_i64()
            .unwrap(),
    };
    Ok(index)
}

pub fn import_zip(zip: &mut ZipArchive<File>, index: DictIndex) -> std::io::Result<LoadedDict> {
    let names = zip.file_names().map(|e| e.to_string()).collect::<Vec<_>>();
    let format = index.format.clone();

    macro_rules! t {
        ($n:expr) => {
            import_type(
                &names.iter().map(|e| e.as_str()).collect(),
                zip,
                Regex::new(concat!($n, r"\_(\d+)\.json")).unwrap(),
                format,
            )
        };
    }

    Ok((
        index,
        t!(r"term\_bank"),
        t!(r"term\_meta\_bank"),
        t!(r"kanji\_bank"),
        t!(r"kanji\_meta\_bank"),
        t!(r"tag\_bank"),
    ))
}

pub fn import_type<T: FromBank>(
    names: &Vec<&str>,
    zip: &mut ZipArchive<File>,
    re: Regex,
    format: i64,
) -> Vec<T> {
    names
        .iter()
        .filter(|e| re.is_match(e))
        .flat_map(|e| parse_bank::<T>(format, zip.by_name(e).unwrap()))
        .collect()
}
