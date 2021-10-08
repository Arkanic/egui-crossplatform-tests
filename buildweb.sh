set -eu

OUT_DIR=dist

rm -rf dist
mkdir dist
cp ./public/* ./${OUT_DIR}/

FOLDER_NAME=${PWD##*/}
CRATE_NAME=$FOLDER_NAME
CRATE_NAME_SNAKE="${CRATE_NAME//-/_}"

# clipboard api
export RUSTFLAGS=--cfg=web_sys_unstable_apis

rm -f ${OUT_DIR}/${CRATE_NAME_SNAKE}_bg.wasm

BUILD=release
cargo build --release -p ${CRATE_NAME} --lib --target wasm32-unknown-unknown

TARGET_NAME="${CRATE_NAME_SNAKE}.wasm"
wasm-bindgen "target/wasm32-unknown-unknown/${BUILD}/${TARGET_NAME}" \
    --out-dir ${OUT_DIR} --no-modules --no-typescript