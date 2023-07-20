#!/usr/bin/sh
set -e

## JCO

# clone the repo if it doesn't exist
if [ ! -d "./jco-repo" ]; then
    git clone https://github.com/bytecodealliance/jco.git jco-repo
fi

# update the repo
cd jco-repo
git reset --hard
git pull

# Turn on instantiation
sed -i -E 's/instantiation: false,/instantiation: true,/' bin/self_build.rs


# generate the files
npm install
npm run build
cd ..

# copy the "generate" component
mkdir -p jco-generate
for file in jco-repo/obj/js-component-bindgen-*; do
    cp $file jco-generate/
done

# remove the ts file, we dont need it
rm jco-generate/*.ts

# copy the wasi shims from jco
rm -rf preview2-shim
cp -r jco-repo/packages/preview2-shim/lib ./preview2-shim

# remove uneeded files from the shim
rm -rf preview2-shim/nodejs
