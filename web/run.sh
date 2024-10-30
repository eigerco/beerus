#!/bin/sh
rm -rf dist/ .parcel-cache/

cd beerus-web
rm -rf target/ pkg/ 

wasm-pack build --target web

cd .. 

npx parcel build index.html
http-server dist/

