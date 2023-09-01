pub mod kanji;
pub mod kanji_meta;
pub mod structured;
pub mod tag;
pub mod term;
pub mod term_meta;

use std::{collections::VecDeque, io::Read};

use serde_json::Value;

pub fn parse_bank<T: FromBank>(format: i32, bank: impl Read) -> Vec<T> {
    serde_json::from_reader::<_, Vec<_>>(bank)
        .unwrap()
        .into_iter()
        .map(|e| FromBank::parse(e, format))
        .collect::<Result<_, _>>()
        .unwrap()
}

pub trait FromBank: Sized {
    fn parse(r: VecDeque<Value>, format: i32) -> serde_json::Result<Self>;
}
