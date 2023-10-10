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
    cargo component build -p $EXAMPLE-guest
done
