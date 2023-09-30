SRC_DIR="./proto/yomishi"
PATH=$PATH:$(pwd)/node_modules/.bin
ES_DST_DIR="./out/ts-protos"
protoc -I $SRC_DIR \
  --es_out $ES_DST_DIR --es_opt target=ts \
  --connect-es_out $ES_DST_DIR --connect-es_opt target=ts \
  $SRC_DIR/*.proto

CONFIG_DST_DIR="./out/ts-config"
mkdir -p $CONFIG_DST_DIR
cargo run --quiet --bin yomishi_config_gen \
  -- ./yomishi-core/config_keys.toml > "$CONFIG_DST_DIR/config.ts"