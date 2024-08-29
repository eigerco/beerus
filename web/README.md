```
cd web/beerus-web
wasm-pack build --target web

cd .. # go back to 'web'
npm i ./beerus-web/pkg
npx parcel build index.html
http-server dist/

## Run the CORS proxy on the same host as a browser
cd etc/proxy
node proxy.js &

## Now open localhost:8080 in a browser
```

