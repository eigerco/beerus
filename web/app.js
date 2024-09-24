const worker = new Worker(new URL('./wrk.js', import.meta.url), { type: 'module' });
worker.onmessage = event => {
    if (!ready) {
        if (event.data === 'OK') {
            dump('log', 'Worker ready');
            ready = true;

            setTimeout(() => {
                post('{"state": {}}');
            }, 1000);
            
            setTimeout(() => {
                post(`{"execute": ${request}}`);
            }, 10000);
        } else {
            dump('log', event.data, 'error');
        }
        return;
    }
    dump('log', event.data);
};
worker.onerror = error => {
    dump('log', error, 'error');
}

var ready;

const config = JSON.stringify({
    network: "mainnet",
    ethereum_url: "http://127.0.0.1:3000/eth-mainnet.g.alchemy.com/v2/nLWc0kd1-CBFNJ57gvB_lVcQpSYCGZS1",
    starknet_url: "http://127.0.0.1:3000/starknet-mainnet.g.alchemy.com/v2/nLWc0kd1-CBFNJ57gvB_lVcQpSYCGZS1"
});
worker.postMessage(config);

const request = JSON.stringify({
    "calldata": [],
    "contract_address": "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    "entry_point_selector": "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60"
});

function post(message) {
    if (!ready) {
        throw new Error('worker not ready');
    }
    dump('log', message);
    worker.postMessage(message);
}

function dump(id, text, style) {
    let div = document.getElementById(id);
    let p = document.createElement('p');
    if (style != undefined) {
        p.className = style;
    }
    if (style === 'error') {
        console.error(text);
    } else {
        console.log(text);
    }
    p.innerText = text;
    div.appendChild(p);
}
