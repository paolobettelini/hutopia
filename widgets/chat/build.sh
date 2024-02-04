#!/bin/bash

# =============
cargo_command="cargo b -j 1 -r" # --release
plugin_folder="../../hutopia-server-space/plugins"
server_crate="chat-plugin-server"
client_crate="chat-plugin-client"
# =============

trap 'echo "Error: Command failed"; exit 1' ERR

# build the pkg using wasm-pack
cd $client_crate
wasm-pack build --target web

cd website
npm install # --dry-run --quiet || npm install # run install only if necessary
npm run build

# mv dist

if [ -d "../../dist" ]; then
    rm -rf ../../dist
fi
mv ./dist ../../

cd ..
cd ..
cd $server_crate

# compile server plugin
eval "$cargo_command"

# Recompile using json message
cargo_command+=" --message-format=json"
build_output=$(eval "$cargo_command")

# Take the path of the .so file and put it in the plugin/ folder
compiled_so_file=$(echo "$build_output" | jq -r "select(.filenames != null and (.package_id | test(\"$server_crate\"))) | .filenames[] | select(endswith(\".so\"))")

cd ..

mkdir -p $plugin_folder
old_plugin="$plugin_folder/$(basename "$compiled_so_file")"
if [ -f "$old_plugin" ]; then
    rm -f "$old_plugin"
fi
mv $compiled_so_file $plugin_folder