#!/usr/bin/sh
set -e

for test in no_bindgen/*; do
  test=$(echo "$test" | sed 's#no_bindgen/##')
  sh skeletons/no_bindgen/run_test.sh "$test"
done

for test in wit_components/*; do
  test=$(echo "$test" | sed 's#wit_components/##')
  sh skeletons/wit_components/run_test.sh "$test"
done

# FIXME: cargo component update seems to have broken wasi components
# see https://github.com/bytecodealliance/cargo-component/issues/105
# for test in wasi_components/*; do
#   test=$(echo "$test" | sed 's#wasi_components/##')
#   sh skeletons/wasi_components/run_test.sh "$test"
# done
