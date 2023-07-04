#!/usr/bin/sh

for test in no_bindgen/*; do
  # create the instance folder if it doesn't exist
  mkdir -p instance

  # copy the "no_bindgen" skeleton
  cp -r skeletons/no_bindgen/plugin ./instance
  cp -r skeletons/no_bindgen/host_sys ./instance
  cp -r skeletons/no_bindgen/host_js ./instance

  # copy the plugin code
  cp $test/plugin.rs instance/plugin/src/lib.rs

  # build the plugin
  cd instance/plugin && cargo rustc --target=wasm32-unknown-unknown -- -C target-feature=+multivalue && cd ../..
  if [ $? -ne 0 ]; then
    echo
    echo "Oh no, there is an error in the $test plugin."
    echo "Inspect the instance for more detail."
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
    echo "Inspect the instance for more detail."
    exit 1
  fi

  # run the js host test
  cd instance/host_js && wasm-pack test --node && cd ../..
  if [ $? -ne 0 ]; then
    echo
    echo "Oh no, there is an error in the $test js host."
    echo "Inspect the instance for more detail."
    exit 1
  fi
done

for test in wit_components/*; do
  # create the instance folder if it doesn't exist
  mkdir -p instance

  # copy the "wit_components" skeleton
  cp -r skeletons/wit_components/plugin ./instance
  cp -r skeletons/wit_components/host_sys ./instance
  cp -r skeletons/wit_components/host_js ./instance

  # copy the protocol
  cp $test/protocol.wit ./instance/protocol.wit

  # copy the plugin code
  cp $test/plugin.rs instance/plugin/src/lib.rs

  # build the plugin
  cd instance/plugin && cargo rustc --target=wasm32-unknown-unknown -- -C target-feature=+multivalue && \
  cd target/wasm32-unknown-unknown/debug && \
  wasm-tools component new wit_components_plugin.wasm -o component.wasm && \
  jco transpile component.wasm --instantiation -o out-dir && \
  cd ../../../../..
  if [ $? -ne 0 ]; then
    echo
    echo "Oh no, there is an error in the $test plugin."
    echo "Inspect the instance for more detail."
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
    echo "Inspect the instance for more detail."
    exit 1
  fi

  # run the js host test
  # cd instance/host_js && wasm-pack test --node && cd ../..
  # if [ $? -ne 0 ]; then
  #   echo
  #   echo "Oh no, there is an error in the $test js host."
  #   echo "Inspect the instance for more detail."
  #   exit 1
  # fi
done

