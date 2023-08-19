mod deinflector;
mod dict_parser;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        deinflector::{deinflect, deinflect_all},
        dict_parser::import_from_path,
    };

    #[test]
    fn test_yomishi() {
        let (title, dict) = import_from_path(Path::new("../local_test_files/index.json")).unwrap();
        println!("imported \"{title}\" with {} terms", dict.len());

        let deinf = include_str!("../../local_test_files/deinflect.json");
        let deinf_list = serde_json::from_str(deinf).unwrap();
        let di = deinflect(&deinf_list, "食べませんでした");
        println!("{:?}", di);

        let di_all = deinflect_all(&deinf_list, "食べませんでした");

        for a in di_all {
            println!("{:?}", a)
        }
    }
}
