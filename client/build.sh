#!/bin/bash

wasm-pack build --release --target=web && \

rm -rf ../src/pkg/

mkdir -p ../src/pkg/

cp -r ./pkg/* ../src/pkg/

# copy the client_bg.wasm to the dist folder since parcel isn't able to process it.
cp  ./pkg/client_bg.wasm ../dist/

# remove the import.meta.url in client.js since the browser complains about it
sed -i -- 's/, import\.meta\.url//g' ../src/pkg/client.js

