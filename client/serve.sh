#!/bin/bash

wasm-pack build --release --target=web && \

mkdir -p ../src/pkg/

cp ./pkg/client.js ../src/pkg/
cp ./pkg/client_bg.wasm ../dist/

# remove the import.meta.url in client.js since the browser complains about it

sed -i -- 's/, import\.meta\.url//g' ../src/pkg/client.js

