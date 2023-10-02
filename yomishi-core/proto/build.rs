use std::path::PathBuf;

use prost_build::Config;
use yomishi_proto_gen::ServiceGen;

fn main() {
    println!("cargo:rerun-if-changed=../../proto/yomishi");
    let src = PathBuf::from("../../proto/yomishi");
    let includes = &[src.clone()];

    let mut config = Config::new();
    config.service_generator(Box::new(ServiceGen));

    config
        .compile_protos(
            &[
                src.join("scan.proto"),
                src.join("config.proto"),
                src.join("anki.proto"),
            ],
            includes,
        )
        .unwrap();
}
