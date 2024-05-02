#!/usr/bin/sh
set -e

# Run from this folder

for test in no_bindgen/*; do
  test=$(echo "$test" | sed 's#no_bindgen/##')
  if [ "$test" != "README.md" ]; then
    sh skeletons/no_bindgen/run_test.sh "$test"
  fi
done

for test in wit_components/*; do
  test=$(echo "$test" | sed 's#wit_components/##')
  if [ "$test" != "README.md" ]; then
    sh skeletons/wit_components/run_test.sh "$test"
  fi
done

for test in wasi_components/*; do
  test=$(echo "$test" | sed 's#wasi_components/##')
  if [ "$test" != "README.md" ]; then
    sh skeletons/wasi_components/run_test.sh "$test"
  fi
done
