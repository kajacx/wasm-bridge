#!/usr/bin/sh

# Run from the "tests" folder and pass the instance name as an argument
test="no_bindgen/$1"

# create the instances/no_bindgen folder if it doesn't exist
mkdir -p instances/no_bindgen

# copy the "no_bindgen" skeleton
cp -r skeletons/no_bindgen/plugin instances/no_bindgen
cp -r skeletons/no_bindgen/host_sys instances/no_bindgen
cp -r skeletons/no_bindgen/host_js instances/no_bindgen

# copy the plugin code
cp $test/plugin.rs instances/no_bindgen/plugin/src/lib.rs

# build the plugin
cd instances/no_bindgen/plugin && cargo rustc --target=wasm32-unknown-unknown -- -C target-feature=+multivalue && cd ../../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test plugin."
  echo "Inspect the instances/no_bindgen for more detail."
  exit 1
fi

# copy the host code
cp $test/host.rs instances/no_bindgen/host_sys/src/host.rs
cp $test/host.rs instances/no_bindgen/host_js/src/host.rs

# run the sys host test
cd instances/no_bindgen/host_sys && cargo run && cd ../../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test sys host."
  echo "Inspect the instances/no_bindgen for more detail."
  exit 1
fi

# run the js host test
cd instances/no_bindgen/host_js && wasm-pack test --node && cd ../../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js host."
  echo "Inspect the instances/no_bindgen for more detail."
  exit 1
fi
