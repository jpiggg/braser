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
    rm -rf pkg-bundler
fi


# Build for all targets
wasm-pack build -t web -d pkg && \
wasm-pack build -t bundler -d pkg-bundler && \
wasm-pack build -t nodejs -d pkg-node

# Get the package name
PKG_NAME=$(jq -r .name pkg/package.json | sed 's/\-/_/g')

mv "pkg/${PKG_NAME}.js" "pkg/${PKG_NAME}_web.js"

# Merge nodejs & browser & bundler packages
cp "pkg-node/${PKG_NAME}.js" "pkg/${PKG_NAME}_node.js"
cp "pkg-bundler/${PKG_NAME}.js" "pkg/${PKG_NAME}.js"

sed "s/require[\(]'\.\/${PKG_NAME}/require\('\.\/${PKG_NAME}_node/" "pkg-bundler/${PKG_NAME}_bg.js" > "pkg/${PKG_NAME}_bg.js"
jq ".files += [\"${PKG_NAME}_bg.js\", \"${PKG_NAME}_web.js\"]" pkg/package.json \
    | jq ".main = \"${PKG_NAME}_node.js\"" > pkg/temp.json
mv pkg/temp.json pkg/package.json
rm -rf pkg-node
rm -rf pkg-bundler