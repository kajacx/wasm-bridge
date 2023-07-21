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

# TODO: Crypto is not defined ...
cp ../random.js browser/random.js

# bundle the files into a single file
esbuild index.js --bundle --outfile=bundled.js

# return the import object from the bundle
head -n -1 bundled.js > bundled_new.js
echo >> bundled_new.js
echo '  return getWasiImports();' >> bundled_new.js
echo '})();' >> bundled_new.js
mv bundled_new.js bundled.js

cd ..


## Jco Rust crates

# copy the files
rm -rf jco-crates/js-component-*
cp -r ../original/jco-crates/js-component-bindgen jco-crates/
cp -r ../original/jco-crates/js-component-bindgen-component jco-crates/

# update the version and edition
sed -i -E 's/version.workspace.*/version = "0.1.0" # MODIFIED, NOT THE REAL VERSION!/' \
jco-crates/js-component-bindgen/Cargo.toml

sed -i -E 's/version.workspace.*/version = "0.1.0" # MODIFIED, NOT THE REAL VERSION!/' \
jco-crates/js-component-bindgen-component/Cargo.toml

sed -i -E 's/version.edition.*/edition = "2021"/' jco-crates/js-component-bindgen/Cargo.toml

sed -i -E 's/version.edition.*/edition = "2021"/' jco-crates/js-component-bindgen-component/Cargo.toml

# update the dependencies
sed -i -E 's'
