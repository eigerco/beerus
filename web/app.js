const worker = new Worker(new URL('./wrk.js', import.meta.url), { type: 'module' });
worker.onmessage = event => {
    dump('log', event.data);
};
worker.onerror = error => {
    dump('log', error, 'error');
}

worker.postMessage(42);

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
