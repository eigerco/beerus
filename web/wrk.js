import init, { get_state } from './beerus-web/pkg/beerus_web.js';

var ready = false;

self.onmessage = async event => {
    if (!ready) {
        await init();
        ready = true;
    }
    console.log('work event: ', event.data);
    let result = await exe();
    self.postMessage(result);
}

async function exe() {
    const config = JSON.stringify({
        network: 'mainnet',
        ethereum_url: 'http://127.0.0.1:3000/eth-mainnet.g.alchemy.com/v2/nLWc0kd1-CBFNJ57gvB_lVcQpSYCGZS1',
        starknet_url: 'http://127.0.0.1:3000/starknet-mainnet.g.alchemy.com/v2/nLWc0kd1-CBFNJ57gvB_lVcQpSYCGZS1'
    });
    console.log("Config: " + config);
    try {
        const state = await get_state(config, post);
        console.log('state: ', state);
        return state;
    } catch (err) {
        console.error('error: ', err);
        return null;
    }
}

function post(url, body) {
    console.log("post: ", url, body);
    const xhr = new XMLHttpRequest();
    xhr.open("POST", url, false);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(body);
    if (xhr.status != 200) {
        console.log("post error: ", xhr.statusText);
        throw new Error(xhr.statusText);
    }
    console.log("post done: ", xhr.responseText);
    return xhr.responseText;
}
