#!/usr/bin/sh

# Run from the "tests" folder and pass the instance name as an argument
test="wit_components/$1"

# Build the guest
sh skeletons/wit_components/build_guest.sh "$1"
if [ $? -ne 0 ]; then
  exit 1
fi


# copy the sys bench skeleton
cp -r skeletons/wit_components/bench_sys instance

# copy the host code
cp $test/bench.rs instance/bench_sys/src/host.rs

# run the sys host bech
cd instance/bench_sys && cargo test --lib --release -- --nocapture && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test sys bench."
  echo "Inspect the instance folder for more detail."
  exit 1
fi


# copy the js bench skeleton
cp -r skeletons/wit_components/bench_js_old instance

# copy the host code
cp $test/bench.rs instance/bench_js_old/src/host.rs

# run the js host bench with old code
cd instance/bench_js_old && wasm-pack test --release --node && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js old bench."
  echo "Inspect the instance folder for more detail."
  exit 1
fi


# copy the new js bench skeleton
cp -r skeletons/wit_components/bench_js_new instance

# copy the host code
cp $test/bench.rs instance/bench_js_new/src/host.rs

# run the new js bench test
cd instance/bench_js_new && wasm-pack test --release --node && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test js new bench."
  echo "Inspect the instance folder for more detail."
  exit 1
fi
