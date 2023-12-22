#!/usr/bin/sh

# Run from the "tests" folder and pass the instance name as an argument
test="wit_components/$1"

# Build the guest
sh skeletons/wit_components/build_guest.sh "$1" "--release"
if [ $? -ne 0 ]; then
  exit 1
fi

# copy the "old" js bench skeleton
cp -r skeletons/wit_components/bench_js instance

# copy the host code
cp $test/bench.rs instance/bench_js/src/host.rs

# run the js bench test
cd instance/bench_js && wasm-pack test --release --node && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js bench."
  echo "Inspect the instance folder for more detail."
  exit 1
fi

# copy the "new" js bench skeleton
cp -r skeletons/wit_components/bench_js_opt instance

# copy the host code
cp $test/bench.rs instance/bench_js_opt/src/host.rs

# run the js bench test
cd instance/bench_js_opt && wasm-pack test --release --node && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js optimized bench."
  echo "Inspect the instance folder for more detail."
  exit 1
fi
