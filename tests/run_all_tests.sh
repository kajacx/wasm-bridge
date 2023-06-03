#!/usr/bin/sh

for test in no_bindgen/*; do
  # test=$(echo $test | sed -E 's#no_bindgen/##')
  # rm -rf instance # && mkdir instance
  # #cd instance && mkdir plugin && cd ..
  # cp -r skeletons/no_bindgen ./instance
  # cp no_bindgen/$test/plugin.rs instance/plugin/src/lib.rs
  
  # clear previous instance
  # rm -rf instance

  # create the instance folder if it doesn't exist
  mkdir -p instance

  # copy the "no_bindgen" skeleton
  cp -r skeletons/no_bindgen/plugin ./instance
  cp -r skeletons/no_bindgen/host_sys ./instance
  cp -r skeletons/no_bindgen/host_js ./instance

  # copy the plugin code
  cp $test/plugin.rs instance/plugin/src/lib.rs

  # build the plugin
  cd instance/plugin && cargo build --target=wasm32-unknown-unknown && cd ../..
  if [ $? -ne 0 ]; then
    echo
    echo "Oh no, there is an error in the $test plugin."
    echo "Inspect the instance for more detail."
    exit 1
  fi

  # copy the host code
  cp $test/host.rs instance/host_sys/src/host.rs
  cp $test/host.rs instance/host_js/src/host.rs

  # run the sys host
  cd instance/host_sys && cargo run && cd ../..
  if [ $? -ne 0 ]; then
    echo
    echo "Oh no, there is an error in the $test sys host."
    echo "Inspect the instance for more detail."
    exit 1
  fi
done
