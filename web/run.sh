#!/bin/sh
cd beerus-web
rm -rf target/ pkg/ .parcel-cache/ 

wasm-pack build --target web

cd .. 

npx parcel build index.html
http-server dist/

