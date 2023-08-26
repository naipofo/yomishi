SRC_DIR="./proto/yomishi"
PATH=$PATH:$(pwd)/node_modules/.bin
ES_DST_DIR="./out/ts-protos"
protoc -I $SRC_DIR \
  --es_out $ES_DST_DIR --es_opt target=ts \
  --connect-es_out $ES_DST_DIR --connect-es_opt target=ts \
  $SRC_DIR/*.proto