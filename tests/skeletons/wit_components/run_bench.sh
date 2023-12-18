#!/usr/bin/sh

# Run from the "tests" folder and pass the instance name as an argument
test="wit_components/$1"

# Build the guest
sh skeletons/wit_components/build_guest.sh "$1"
if [ $? -ne 0 ]; then
  exit 1
fi

# copy the "wit_components" skeleton
cp -r skeletons/wit_components/host_js_bench instance

# copy the host code
cp $test/bench.rs instance/host_js_bench/src/host.rs

# run the js bench test
cd instance/host_js_bench && wasm-pack test --node && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js bench."
  echo "Inspect the instance folder for more detail."
  exit 1
fi
