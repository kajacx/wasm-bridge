#!/bin/bash

set -e

if [[ $1 ]]; then
    echo "-> Building $1"
    EXAMPLES=$1
else
    EXAMPLES=`ls `
fi


# Build the guest
for example in $EXAMPLES; do
    if [[ ! -d $example ]]; then
        continue
    fi

    echo "-> Running $example"
    cargo component build --target wasm32-unknown-unknown -p example-$example-guest
    cargo run -p example-$example-host
done
