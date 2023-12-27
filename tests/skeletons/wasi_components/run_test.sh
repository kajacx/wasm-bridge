#!/usr/bin/sh

# Run from the "tests" folder and pass the instance name as an argument
test="wasi_components/$1"

# create the instance folder if it doesn't exist
mkdir -p instance

# copy the "wasi_components" skeleton
cp -r skeletons/wasi_components/guest instance
cp -r skeletons/wasi_components/host_sys instance
cp -r skeletons/wasi_components/host_js instance

# copy the protocol
cp $test/protocol.wit instance/protocol.wit
if [ $? -ne 0 ]; then
  echo "Non-existing test: $test"
  exit 1
fi

# copy the guest code
cp $test/guest.rs instance/guest/src/lib.rs

# build the guest
cd instance/guest && cargo component build --release && cd ../..
if [ $? -ne 0 ]; then
  echo
  echo "Oh no, there is an error in the $test guest."
  echo "Inspect the instance folder for more detail."
  exit 1
fi

# copy the host code
cp $test/host.rs instance/host_sys/src/host.rs
cp $test/host.rs instance/host_js/src/host.rs

if [ "$test" = "wasi_components/io_redirect" ]; then
  # run test and capture its output

  # run the sys host test
  cd instance/host_sys && cargo run > out.txt 2> err.txt && cd ../..
  if [ $? -ne 0 ]; then
    cat *.txt
    echo
    echo "Oh no, there is an error in the $test sys host."
    echo "Inspect the instance folder for more detail."
    exit 1
  fi
  cat instance/host_sys/*.txt
  
  # test the files for output
  print1=$(grep PRINT_OUT_1 instance/host_sys/out.txt)
  print2=$(grep PRINT_ERR_1 instance/host_sys/err.txt)
  
  no_print1=$(grep NO_PRINT instance/host_sys/out.txt)
  no_print2=$(grep NO_PRINT instance/host_sys/err.txt)

  if [ "$print1" = "" ]; then
    echo "Sys host should have printed PRINT_OUT_1 to stdout"
    exit 1
  fi
  if [ "$print2" = "" ]; then
    echo "Sys host should have printed PRINT_ERR_1 to stderr"
    exit 1
  fi

  if [ "$no_print1" != "" ]; then
    echo "Sys host should NOT have printed NO_PRINT to stdout"
    exit 1
  fi
  if [ "$no_print2" != "" ]; then
    echo "Sys host should NOT have printed NO_PRINT to stderr"
    exit 1
  fi

  # run the js host test
  cd instance/host_js && wasm-pack test --node > out.txt 2> err.txt && cd ../..
  if [ $? -ne 0 ]; then
    cat *.txt
    echo
    echo "Oh no, there is an error in the $test js host."
    echo "Inspect the instance folder for more detail."
    exit 1
  fi
  cat instance/host_js/*.txt

  # test the files for output
  print1=$(grep PRINT_OUT_1 instance/host_js/out.txt)
  print2=$(grep PRINT_ERR_1 instance/host_js/err.txt)
  
  no_print1=$(grep NO_PRINT instance/host_js/out.txt)
  no_print2=$(grep NO_PRINT instance/host_js/err.txt)

  if [ "$print1" = "" ]; then
    echo "Js host should have printed PRINT_OUT_1 to stdout"
    exit 1
  fi
  if [ "$print2" = "" ]; then
    echo "Js host should have printed PRINT_ERR_1 to stderr"
    exit 1
  fi

  if [ "$no_print1" != "" ]; then
    echo "Js host should NOT have printed NO_PRINT to stdout"
    exit 1
  fi
  if [ "$no_print2" != "" ]; then
    echo "Js host should NOT have printed NO_PRINT to stderr"
    exit 1
  fi

else
  # run test normally

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
fi
