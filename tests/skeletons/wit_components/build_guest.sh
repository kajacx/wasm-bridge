#!/usr/bin/sh

# Run from the "tests" folder and pass the instance name as an argument
test="wit_components/$1"

# create the instance folder if it doesn't exist
mkdir -p instance

# copy the "wit_components" skeleton
cp -r skeletons/wit_components/guest instance

# copy the protocol
cp $test/protocol.wit instance/protocol.wit
if [ $? -ne 0 ]; then
  echo "Non-existing test: $test"
  exit 1
fi

# copy the guest code
# FIXME: uncomment this after the code is debugged
# cp $test/guest.rs instance/guest/src/lib.rs

# expand the code
cd instance/guest && ./expand.sh && cd ../..

# build the guest
cd instance/guest && cargo component build --target wasm32-unknown-unknown && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test guest."
  echo "Inspect the instance folder for more detail."
  exit 1
fi
