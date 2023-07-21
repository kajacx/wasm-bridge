#!/usr/bin/sh
set -e

## First, clone and update the repo

# clone the repo if it doesn't exist
if [ ! -d "./jco-repo" ]; then
    git clone https://github.com/bytecodealliance/jco.git jco-repo
fi

# update the repo
cd jco-repo
git reset --hard
git pull
cd ..


## Copy the Rust crates

rm -rf jco-crates/js-component-bindgen
rm -rf jco-crates/js-component-bindgen-component

cp -r jco-repo/crates/js-component-bindgen jco-crates/
cp -r jco-repo/crates/js-component-bindgen-component jco-crates/



## Preview shim

# copy the wasi shims from jco
rm -rf preview2-shim/browser
rm -rf preview2-shim/http
cp -r jco-repo/packages/preview2-shim/lib/browser ./preview2-shim/
cp -r jco-repo/packages/preview2-shim/lib/http ./preview2-shim/

# remove uneeded files from the shim
rm -rf preview2-shim/nodejs
