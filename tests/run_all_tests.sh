#!/usr/bin/sh
set -e

for test in no_bindgen/*; do
  test=$(echo "$test" | sed 's#no_bindgen/##')

  sh skeletons/no_bindgen/run_test.sh "$test"
done

sh run_wit_tests.sh
if [ $? -ne 0 ]; then
  exit 1
fi


