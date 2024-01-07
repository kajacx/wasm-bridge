#!/usr/bin/sh
set -e

version=$(grep '^version' Cargo.toml | sed -E 's#.*?"(.*?)".*#\1#')

echo You are about to release version $version.
echo

echo Please check that all version references have been updated:
grep '^wasm-bridge' Cargo.toml
echo

echo Please make sure that the CHANGELOG has today"'"s date
grep "$version" CHANGELOG.md
echo

echo Also, review the git status
git status
echo

echo Looks good? "(y/n)"
read resp
if [ "$resp" != "y" ] && [ "$resp" != "Y" ]; then
    echo "Never mind then, bye"
    exit
fi

echo Ok then, let me check that automated tests pass ...
sh run_checks.sh

echo Everything ok
echo FINAL QUESTION: RELEASE A NEW VERSION TO crates.io? "(y/n)"
read resp
if [ "$resp" != "y" ] && [ "$resp" != "Y" ]; then
    echo "Never mind then, bye"
    exit
fi
echo 

echo Releasing crates
cargo publish -p wasm-bridge-macros
cargo publish -p wasm-bridge-js
cargo publish -p wasm-bridge
cargo publish -p wasm-bridge-wasi
echo

echo Creating git tag
git tag "v$version"
git push origin "v$version"
echo

echo "Everything done, version $version successfully released :-)"
