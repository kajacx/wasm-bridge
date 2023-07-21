#!/usr/bin/sh

# Optimization to speed up warnings checks
export RUSTFLAGS="-D warnings"

# Run from the "tests" folder and pass the instance name as an argument
test="wit_components/$1"

# create the instances/wit_components folder if it doesn't exist
mkdir -p instances/wit_components

# copy the "wit_components" skeleton
cp -r skeletons/wit_components/guest instances/wit_components
cp -r skeletons/wit_components/host_sys instances/wit_components
cp -r skeletons/wit_components/host_js instances/wit_components

# copy the protocol
cp $test/protocol.wit instances/wit_components/protocol.wit

# copy the guest code
cp $test/guest.rs instances/wit_components/guest/src/lib.rs

# build the guest
cd instances/wit_components/guest && cargo rustc --target=wasm32-unknown-unknown -- -C target-feature=+multivalue && \
cd target/wasm32-unknown-unknown/debug && \
wasm-tools component new wit_components_guest.wasm -o component.wasm && \
cd ../../../../../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test guest."
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
