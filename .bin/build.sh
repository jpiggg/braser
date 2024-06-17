# #!/bin/bash

set -e

# Check if jq is installed
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

# Clean previous packages
if [ -d "pkg" ]; then
    rm -rf pkg
fi

if [ -d "pkg-node" ]; then
    rm -rf pkg-node
fi


if [ -d "pkg-bundler" ]; then
    rm -rf pkg-node
fi

# Build for all targets
wasm-pack build -t bundler -d pkg && \
wasm-pack build -t nodejs -d pkg-node && \
wasm-pack build -t web -d pkg-web

# Get the package name
PKG_NAME=$(jq -r .name pkg/package.json | sed 's/\-/_/g')

# Merge nodejs & bundler & bundler packages
cp "pkg-node/${PKG_NAME}.js" pkg/index.cjs.js
cp "pkg/${PKG_NAME}.js" pkg/index.esm.js
cp "pkg-web/${PKG_NAME}.js" pkg/index.iife.js

rm -f "pkg/${PKG_NAME}.js" 

sed -i '' -e '3i\'$'\n''imports["\.\/'"${PKG_NAME}"'_bg.js"] = module.exports;' pkg/index.cjs.js
jq ".files += [\"${PKG_NAME}_bg.js\"]" pkg/package.json \
    | jq ".main = \"index.cjs.js\"" > pkg/temp.json
    # | jq ".module = \"index.esm.js\"" > pkg/temp.json
mv pkg/temp.json pkg/package.json

sed -i '' -e 's/"module".*$/"module": "index.esm.js",/' pkg/package.json

rm -rf pkg-node
rm -rf pkg-web

# @TODO: minify build for esm & iife with webpack
# if !command -v ./.bin/node_modules/.bin/webpack &> /dev/null
# then
#     npm i
# fi