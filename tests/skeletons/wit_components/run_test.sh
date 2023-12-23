#!/usr/bin/sh

# Run from the "tests" folder and pass the instance name as an argument
test="wit_components/$1"

# Build the guest
sh skeletons/wit_components/build_guest.sh "$1"
if [ $? -ne 0 ]; then
  exit 1
fi

# copy the "wit_components" skeleton
cp -r skeletons/wit_components/host_sys instance
cp -r skeletons/wit_components/host_js instance
cp -r skeletons/wit_components/host_js_opt instance

# copy the host code
cp $test/host.rs instance/host_sys/src/host.rs
cp $test/host.rs instance/host_js/src/host.rs
cp $test/host.rs instance/host_js_opt/src/host.rs

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

# run the js host optimized test
cd instance/host_js_opt && wasm-pack test --node && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js optimized host."
  echo "Inspect the instance folder for more detail."
  exit 1
fi

