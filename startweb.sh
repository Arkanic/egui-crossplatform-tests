OUT_DIR=dist

set -eu

echo "8080"

(cd ${OUT_DIR} && basic-http-server --addr 127.0.0.1:8080 .)