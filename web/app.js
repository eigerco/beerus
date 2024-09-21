const worker = new Worker(new URL('./wrk.js', import.meta.url), { type: 'module' });
worker.onmessage = event => {
    console.log('main event: ', event.data);
};
worker.postMessage(42);

// dump(div, "State: " + state);
// dump(div, "Error: " + err, 'error');
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
