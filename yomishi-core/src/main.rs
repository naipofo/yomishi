use crate::dict_parser::parse_dict;

mod dict_parser;

fn main() {
    println!("Hello, world!");

    let index = include_str!("../../local_test_files/index.json");
    let terms = include_str!("../../local_test_files/term_bank_1.json");

    let dict = parse_dict(index, terms);
}
