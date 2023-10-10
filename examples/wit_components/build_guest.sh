#!/bin/bash

set -e

if [[ $1 ]]; then
    echo "-> Building $1"
    EXAMPLES=$1
else
    EXAMPLES=`ls guest`
fi


# Build the guest
for EXAMPLE in $EXAMPLES; do
    echo "-> Building $EXAMPLE"
    cargo component build --target wasm32-unknown-unknown -p $EXAMPLE-guest
done
