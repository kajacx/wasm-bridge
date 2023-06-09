#!/usr/bin/sh

# Optimization to speed up warnings checks
export RUSTFLAGS="-D warnings"

# Run from the "tests" folder and pass the instance name as an argument
test="wit_components/$1"

# create the instances/wit_components folder if it doesn't exist
mkdir -p instances/wit_components

# copy the "wit_components" skeleton
cp -r skeletons/wit_components/plugin instances/wit_components
cp -r skeletons/wit_components/host_sys instances/wit_components
cp -r skeletons/wit_components/host_js instances/wit_components

# copy the protocol
cp $test/protocol.wit instances/wit_components/protocol.wit

# copy the plugin code
cp $test/plugin.rs instances/wit_components/plugin/src/lib.rs

# build the plugin
cd instances/wit_components/plugin && cargo rustc --target=wasm32-unknown-unknown -- -C target-feature=+multivalue && \
cd target/wasm32-unknown-unknown/debug && \
wasm-tools component new wit_components_plugin.wasm -o component.wasm && \
jco transpile component.wasm --instantiation -o out-dir && \
cargo run --manifest-path ../../../../../../../crates/wasm-bridge-cli/Cargo.toml out-dir -o out-dir.zip && \
cargo run --manifest-path ../../../../../../../crates/wasm-bridge-cli/Cargo.toml out-dir -u component.wasm -o universal.zip && \
cd ../../../../../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test plugin."
  echo "Inspect the instances/wit_components for more detail."
  exit 1
fi

# copy the host code
cp $test/host.rs instances/wit_components/host_sys/src/host.rs
cp $test/host.rs instances/wit_components/host_js/src/host.rs

# run the sys host test
cd instances/wit_components/host_sys && cargo run && cd ../../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test sys host."
  echo "Inspect the instances/wit_components for more detail."
  exit 1
fi

# run the js host test
cd instances/wit_components/host_js && wasm-pack test --node && cd ../../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js host."
  echo "Inspect the instances/wit_components for more detail."
  exit 1
fi
