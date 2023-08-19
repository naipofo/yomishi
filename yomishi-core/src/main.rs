mod deinflector;
mod dict_parser;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{
        deinflector::{deinflect, deinflect_all},
        dict_parser::parse_dict,
    };

    #[test]
    fn test_yomishi() {
        let index = include_str!("../../local_test_files/index.json");
        let terms = include_str!("../../local_test_files/term_bank_1.json");

        let deinf = include_str!("../../local_test_files/deinflect.json");

        let dict = parse_dict(index, terms);

        let deinf_list = serde_json::from_str(deinf).unwrap();

        let di = deinflect(&deinf_list, "食べませんでした");
        println!("{:?}", di);

        let di_all = deinflect_all(&deinf_list, "食べませんでした");

        for a in di_all {
            println!("{:?}", a)
        }
    }
}
