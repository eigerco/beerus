```
cd web/beerus-web
wasm-pack build --target web

# go back to 'web'
cd ..
npx parcel build index.html
http-server dist/

## Keep CORS proxy running
cd etc/proxy
node proxy.js &

## Now open localhost:8080
```

One-liner for full build & serve:

```
cd beerus-web && rm -rf target/ pkg/ .parcel-cache/ && wasm-pack build --target web && cd .. && npx parcel build index.html && http-server dist/
```
