fn main() {
    yomishi_config_gen::compile_config_to_file(
        include_str!("../config_keys.toml"),
        &std::env::var("OUT_DIR").unwrap(),
    );
}
