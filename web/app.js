document.addEventListener("DOMContentLoaded", () => {
    const chatLog = document.getElementById("chat-log");
    const input = document.getElementById("json-input");
    const sendBtn = document.getElementById("send-btn");
    const templateBtns = document.querySelectorAll('.template-btn');
    const alchemyKeyInput = document.getElementById("alchemy-key");
    const initBtn = document.getElementById("init-btn");
    const statusSpan = document.getElementById("status");
    const proxySpan = document.getElementById("proxy");
    const modalOverlay = document.getElementById("modal-overlay");
    const infoBtn = document.getElementById("info-btn");
    const proxyBtn = document.getElementById("proxy-btn");
    let messageId = 1;

    const statusIcons = {
        unknown: '❓',
        pending: '⏳',
        ready: '✅',
        error: '❌'
    };

    var ready;
    const worker = new Worker(new URL('./wrk.js', import.meta.url), { type: 'module' });
    worker.onmessage = event => {
        if (!ready) {
            if (event.data === 'OK') {
                ready = true;
                statusSpan.innerText = statusIcons.ready;
            } else {
                console.error(event.data);
                statusSpan.innerText = statusIcons.error;
            }
            return;
        }

        try {
            let json = JSON.parse(event.data);
            let responseContent = document.getElementById(json.id);
            delete json.id;
            let response = formatJSON(JSON.stringify(json));
            responseContent.innerHTML = response;

            if (json.hasOwnProperty('error')) {
                console.error(json['error']);
                responseContent.parentElement.setAttribute("style", "border-left-color:#FF0000");
            }
        } catch (e) {
            console.error(e);
        }
    };

    worker.onerror = error => {
        console.error(error);
    }

    function sendMessage(userMessage) {
        if (!ready || userMessage.trim() === "") return;

        addMessagePair(userMessage, messageId);

        let message = appendId(userMessage, messageId);
        worker.postMessage(message);

        messageId++;
        input.value = '';
    }

    function appendId(message, id) {
        let object = JSON.parse(message);
        object.id = id;
        return JSON.stringify(object);
    }

    function addMessagePair(userMessage, id) {
        const messagePairDiv = document.createElement("div");
        messagePairDiv.classList.add("message-pair");

        const requestMessage = createMessageDiv(formatJSON(userMessage), id, "request");
        const responseMessage = createMessageDiv(statusIcons.pending, id, "response");
        messagePairDiv.appendChild(requestMessage);
        messagePairDiv.appendChild(responseMessage);
        chatLog.prepend(messagePairDiv);
    }

    function createMessageDiv(messageContent, id, type) {
        const messageDiv = document.createElement("div");
        messageDiv.classList.add("message", type);

        const content = document.createElement("div");
        if (type === "response") {
            content.id = id;
        }
        content.classList.add("content");
        content.innerHTML = messageContent;

        const messageIdDiv = document.createElement("div");
        messageIdDiv.classList.add("id");
        messageIdDiv.textContent = `#${id}`;

        messageDiv.appendChild(content);
        messageDiv.appendChild(messageIdDiv);

        return messageDiv;
    }

    sendBtn.addEventListener("click", () => sendMessage(input.value));
    input.addEventListener("keypress", (e) => {
        if (e.key === "Enter" && e.shiftKey) {
            sendMessage(input.value);
            e.preventDefault(); // Prevents default Enter behavior (add new line)
        }
    });

    templateBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            input.value = JSON.stringify(JSON.parse(btn.dataset.template), null, 2);
        });
    });

    function formatJSON(jsonString) {
        try {
            const obj = JSON.parse(jsonString);
            return syntaxHighlight(JSON.stringify(obj, null, 2));
        } catch (e) {
            return jsonString; // Return input string if not a valid JSON
        }
    }

    function syntaxHighlight(json) {
        json = json.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
        return json.replace(/("(\\u[\da-fA-F]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|\d+)/g, function (match) {
            let cls = 'json-value';
            if (/^"/.test(match)) {
                if (/:$/.test(match)) {
                    cls = 'json-key';
                } else {
                    cls = 'json-string';
                }
            } else if (/true|false/.test(match)) {
                cls = 'json-boolean';
            }
            return `<span class="${cls}">${match}</span>`;
        });
    }

    input.addEventListener('input', function () {
        try {
            const formattedJSON = JSON.stringify(JSON.parse(input.value), null, 2);
            input.value = formattedJSON;
        } catch (e) {
            // ignore if the input is not a valid JSON yet
        }
    });

    initBtn.addEventListener("click", () => {
        if (ready) {
            return;
        }
        const alchemyKey = alchemyKeyInput.value;
        if (!alchemyKey) {
            console.log("Alchemy key is empty");
            return;
        }
        const config = JSON.stringify({
            ethereum_url: `http://127.0.0.1:3000/eth-mainnet.g.alchemy.com/v2/${alchemyKey}`,
            starknet_url: `http://127.0.0.1:3000/starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/${alchemyKey}`
        });
        worker.postMessage(config);
        statusSpan.innerText = statusIcons.pending;
    });

    modalOverlay.addEventListener("click", (e) => {
        if (e.target === modalOverlay) {
            modalOverlay.style.display = "none";
        }
    });

    function timeout(ms, promise) {
        return new Promise((resolve, reject) => {
          const timer = setTimeout(() => {
            reject(new Error('TIMEOUT'))
          }, ms)
      
          promise
            .then(value => {
              clearTimeout(timer)
              resolve(value)
            })
            .catch(reason => {
              clearTimeout(timer)
              reject(reason)
            })
        });
    }

    function checkProxy() {
        proxySpan.innerText = statusIcons.pending;
        timeout(1000, fetch('http://127.0.0.1:3000/check'))
            .then(response => response.text())
            .then(response => {
                console.log('Proxy:', response);
                if (response.trim() === 'ready') {
                    proxySpan.innerText = statusIcons.ready;
                } else {
                    proxySpan.innerText = statusIcons.unknown;
                }
            })
            .catch((e) => {
                console.error('Proxy:', e);
                proxySpan.innerText = statusIcons.error;
            });
    }

    proxyBtn.addEventListener("click", checkProxy);

    modalOverlay.style.display = "flex";
    infoBtn.addEventListener("click", () => {
        modalOverlay.style.display = "flex";
    });

    document.addEventListener("keydown", (e) => {
        if (e.key === "Escape") {
            modalOverlay.style.display = "none";
        }
    });

    checkProxy();
});
