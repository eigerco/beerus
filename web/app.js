var ready;
var id = 0;

const worker = new Worker(new URL('./wrk.js', import.meta.url), { type: 'module' });
worker.onmessage = event => {
    if (!ready) {
        if (event.data === 'OK') {
            dump('log', 'worker ready');
            ready = true;
            set_status(event.data);
        } else {
            dump('log', event.data, 'error');
            set_status(event.data);
        }
        return;
    }

    try {
        let json = JSON.parse(event.data);
        let pretty = JSON.stringify(json, null, 2);
        if (json.hasOwnProperty('error')) {
            dump('log', '<<< ' + pretty, 'error');
        } else {
            dump('log', '<<< ' + pretty);
        }
    } catch (e) {
        console.error(e);
        dump('log', '[invalid JSON] <<< ' + event.data, 'error');
    }
};
worker.onerror = error => {
    dump('log', error, 'error');
}

const request = {
    "calldata": [],
    "contract_address": "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    "entry_point_selector": "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60"
};

function post(message) {
    message.id = id;
    let payload = JSON.stringify(message, null, 2);
    get('txt').value = payload;
}

function dump(id, text, style) {
    let div = document.getElementById(id);
    let p = document.createElement('pre');
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

function get(id) {
    return document.getElementById(id);
} 

function run() {
    var key = get('key');
    var setup = get('setup');
    setup.onclick = () => {
        const config = JSON.stringify({
            network: "mainnet",
            ethereum_url: `http://127.0.0.1:3000/eth-mainnet.g.alchemy.com/v2/${key.value}`,
            starknet_url: `http://127.0.0.1:3000/starknet-mainnet.g.alchemy.com/v2/${key.value}`
        });
        worker.postMessage(config);
        set_status('wait');
    }

    var state = get('get');
    state.onclick = () => {
        post({"state": {}});
    };

    var exe = get('exe');
    exe.onclick = () => {
        post({"execute": request});
    };

    get('clear').onclick = () => {
        let log = get('log');
        log.replaceChildren();
    };

    var run = get('run');
    run.disabled = true;
    run.onclick = () => {
        let payload = get('txt').value;
        if (!ready) {
            throw new Error('worker not ready');
            return;
        }
        dump('log', '>>> ' + payload);
        worker.postMessage(payload);
        id += 1;
    }

    get('txt').value = '';
}

function set_status(message) {
    var status = get('status');
    if (message === 'OK') {
        status.innerText = 'READY';
        status.classList.remove('status-wait');
        status.classList.add('status-ready');
        get('setup').disabled = true;
        get('run').disabled = false;
    } else if (message === 'wait') {
        status.innerText = 'WAIT';
        status.classList.add('status-wait');
        get('setup').disabled = true;
        get('run').disabled = true;
    } else {
        status.innerText = 'ERROR';
        status.classList.remove('status-wait');
        status.classList.add('status-error');
        get('setup').disabled = false;
        get('run').disabled = true;
    }
}

run();
