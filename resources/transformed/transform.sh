#!/usr/bin/sh
set -e

## Wasi shim

# copy the files
rm -rf /browser
rm -rf preview2-shim/http

cp -r ../original/preview2-shim/browser preview2-shim
cp -r ../original/preview2-shim/http preview2-shim

# change the import path
cd preview2-shim
sed -i -E 's#@bytecodealliance/preview2-shim#../browser/#' http/wasi-http.js

# bundle the files into a single file
esbuild index.js --bundle --outfile=bundled.js

# return the import object from the bundle
head -n -1 bundled.js > bundled_new.js
echo >> bundled_new.js
echo '  return getWasiImports();' >> bundled_new.js
echo '})();' >> bundled_new.js
mv bundled_new.js bundled.js

cd ..


## Jco transpile component

# copy the files
rm -rf jco-generate
cp -r ../original/jco-generate ./jco-generate

# TODO: this should not be needed because of noWasiShim
# fix imports 
sed -i -E 's#@bytecodealliance/preview2-shim/##' jco-generate/js-component-bindgen-component.js

# for now, convert jco with wasm-bridge-cli
cargo run --manifest-path ../../crates/wasm-bridge-cli/Cargo.toml -- jco-generate -o jco-web.zip
