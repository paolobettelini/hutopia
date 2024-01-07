#!/bin/bash

# =============
cargo_command="cargo b -j 1" # --release
# =============

# build the pkg using wasm-pack
cd example-plugin-client
CARGO_TARGET_DIR="/home/paolo/Desktop/rust_target" wasm-pack build --target web

# remove useless files
cd ./pkg
rm *.ts
rm *.json
rm .gitignore
cd ..

# mv pkg
if [ -d "../pkg" ]; then
    rm -rf ../pkg
fi
mv ./pkg ../

cd ..
cd example-plugin-server

# compile server plugin
eval "$cargo_command"

# Recompile using json message
cargo_command+=" --message-format=json"
build_output=$(eval "$cargo_command")

# Take the path of the .so file and put it in the plugin/ folder
compiled_so_file=$(echo "$build_output" | jq -r 'select(.filenames != null and (.package_id | test("example-plugin-server"))) | .filenames[] | select(endswith(".so"))')

cd ..

mv $compiled_so_file ../../hutopia-server-space/plugins