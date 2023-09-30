use std::{env, fs};

fn main() {
    yomishi_config_gen::compile_ts_config_to_stdout(
        &fs::read_to_string(env::args().nth(1).unwrap()).unwrap(),
    );
}
