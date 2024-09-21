```
cd web/beerus-web
wasm-pack build --target web

# go back to 'web'
cd ..
npx parcel build index.html
http-server dist/

## Run the CORS proxy locally
cd etc/proxy
node proxy.js &

## Now open localhost:8080 in a browser
```

One-liner for full build & serve:

```
cd beerus-web && rm -rf target/ pkg/ .parcel-cache/ && wasm-pack build --target web && cd .. && npx parcel build index.html && http-server dist/
```
