import init, { get_state } from './node_modules/beerus/beerus_web.js';

async function run() {
    await init();
    const div = document.getElementById('log');

    const config = JSON.stringify({
        network: 'mainnet',
        ethereum_url: 'http://127.0.0.1:3000/eth-mainnet.g.alchemy.com/v2/nLWc0kd1-CBFNJ57gvB_lVcQpSYCGZS1',
        starknet_url: 'http://127.0.0.1:3000/starknet-mainnet.g.alchemy.com/v2/nLWc0kd1-CBFNJ57gvB_lVcQpSYCGZS1'
    });
    console.log("Config: " + config);
    try {
        const state = await get_state(config);
        dump(div, "State: " + state);
    } catch (err) {
        dump(div, "Error: " + err, 'error');
    }
}

function dump(div, text, style) {
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

run();
