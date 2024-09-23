import init, { set_panic_hook, Beerus } from './beerus-web/pkg/beerus_web.js';

var initialized = false;
var configured = false;
var client;

self.onmessage = async event => {
    console.log('worker: ', event.data);
    if (!initialized) {
        await init();
        set_panic_hook();
        initialized = true;
    }
    if (!configured) {
        try {
            let beerus = await new Beerus(event.data, post);
            console.log('Beerus instance created');
            client = beerus;
            configured = true;
            self.postMessage('OK');
            return;
        } catch (e) {
            console.log('failed to create Beerus instance: ', e);
            return;
        }
    }
    let request = JSON.parse(event.data);
    if (request.hasOwnProperty('state')) {
        let state = await client.get_state();
        self.postMessage(state);
    } else if (request.hasOwnProperty('execute')) {
        let req = JSON.stringify(request['execute']);
        let result = await client.execute(req);
        self.postMessage(result);
    } else {
        console.error('worker: unknown request: ', event.data);
        self.postMessage('{"error": "unknown request"}');
    }
}

function post(url, body) {
    // console.log("post: ", url, body);
    const xhr = new XMLHttpRequest();
    xhr.open("POST", url, false);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(body);
    if (xhr.status != 200) {
        // console.log("post error: ", xhr.statusText);
        throw new Error(xhr.statusText);
    }
    // console.log("post done: ", xhr.responseText);
    return xhr.responseText;
}
