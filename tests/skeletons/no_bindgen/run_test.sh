#!/usr/bin/sh

# Optimization to speed up warnings checks
export RUSTFLAGS="-D warnings"

# Run from the "tests" folder and pass the instance name as an argument
test="no_bindgen/$1"

# create the instance folder if it doesn't exist
mkdir -p instance

# copy the "no_bindgen" skeleton
cp -r skeletons/no_bindgen/guest instance
cp -r skeletons/no_bindgen/host_sys instance
cp -r skeletons/no_bindgen/host_js instance

# copy the guest code
cp $test/guest.rs instance/guest/src/lib.rs

# build the guest
cd instance/guest && cargo rustc --target=wasm32-unknown-unknown -- -C target-feature=+multivalue && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test guest."
  echo "Inspect the instance folder for more detail."
  exit 1
fi

# copy the host code
cp $test/host.rs instance/host_sys/src/host.rs
cp $test/host.rs instance/host_js/src/host.rs

# run the sys host test
cd instance/host_sys && cargo run && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test sys host."
  echo "Inspect the instance folder for more detail."
  exit 1
fi

# run the js host test
cd instance/host_js && wasm-pack test --node && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js host."
  echo "Inspect the instance folder for more detail."
  exit 1
fi
