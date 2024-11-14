document.addEventListener('DOMContentLoaded', () => {
	const chatLog = document.getElementById('chat-log');
	const input = document.getElementById('json-input');
	const sendBtn = document.getElementById('send-btn');
	const templateBtns = document.querySelectorAll('.template-btn');
	const alchemyKeyInput = document.getElementById('alchemy-key');
	const initBtn = document.getElementById('init-btn');
	const statusSpan = document.getElementById('status');
	const proxySpan = document.getElementById('proxy');
	const modalOverlay = document.getElementById('modal-overlay');
	const infoBtn = document.getElementById('info-btn');
	const helpBtn = document.getElementById('help-btn');
	const proxyBtn = document.getElementById('proxy-btn');
	const closeBtn = document.getElementById('close-btn');
	const alchemyWindow = document.getElementById('init');
	const terminalWindow = document.getElementById('terminal');
	const terminalWindowHead = document.getElementById('terminal-head');
	const terminalWindowContent = document.getElementById('terminal-content');
	const helpWindow = document.getElementById('help');


	let messageId = 1;

	const statusIcons = {
		unknown:
			'<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g opacity="0.24" clip-path="url(#clip0_7_15)"><path d="M12 21C16.9706 21 21 16.9706 21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21Z" stroke="black" stroke-width="2" stroke-miterlimit="10" stroke-linecap="square"/><path d="M22 2L2 22" stroke="black" stroke-width="2" stroke-miterlimit="10" stroke-linecap="square"/></g><defs><clipPath id="clip0_7_15"><rect width="24" height="24" fill="white"/></clipPath></defs></svg>',
		pending:
			'<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g opacity="0.24"><path opacity="0.4" d="M5 23C5 23 8 15 12 15C16 15 19 23 19 23H5Z" fill="black"/><path d="M3 1H21" stroke="black" stroke-width="2" stroke-miterlimit="10" stroke-linecap="square"/><path d="M3 23H21" stroke="black" stroke-width="2" stroke-miterlimit="10" stroke-linecap="square"/><path d="M5 23V18C5 15 9 12 9 12C9 12 5 9 5 6V1" stroke="black" stroke-width="2" stroke-miterlimit="10"/><path d="M19 1V6C19 9 15 12 15 12C15 12 19 15 19 18V23" stroke="black" stroke-width="2" stroke-miterlimit="10"/></g></svg>',
		ready: '<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 21C16.9706 21 21 16.9706 21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21Z" stroke="#CDFFC5" stroke-width="2" stroke-miterlimit="10" stroke-linecap="square"/><circle cx="12" cy="12" r="6" fill="#56F53D"/></svg>',
		error: '<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 21C16.9706 21 21 16.9706 21 12C21 7.02944 16.9706 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21Z" stroke="#FED8D8" stroke-width="2" stroke-miterlimit="10" stroke-linecap="square"/><circle cx="12" cy="12" r="6" fill="#F53D3D"/></svg>',
	};

	var ready;
	const worker = new Worker(new URL('./wrk.js', import.meta.url), {
		type: 'module',
	});
	worker.onmessage = (event) => {
		if (!ready) {
			if (event.data === 'OK') {
				ready = true;
				statusSpan.innerHTML = statusIcons.ready;
				alchemyWindow.classList.add('remove');
				helpWindow.classList.add('show');
				terminalWindow.classList.remove('hidden');
			} else {
				console.error(event.data);
				statusSpan.innerHTML = statusIcons.error;
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
				// responseContent.parentElement.setAttribute(
				// 	'style',
				// 	'border-left-color:#FF0000'
				// );
				responseContent.parentElement.classList.add('error');
			}
		} catch (e) {
			console.error(e);
		}
	};

	worker.onerror = (error) => {
		console.error(error);
	};

	function sendMessage(userMessage) {
		if (!ready || userMessage.trim() === '') return;

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
		const messagePairDiv = document.createElement('div');
		messagePairDiv.classList.add('message-pair');

		const requestMessage = createMessageDiv(
			formatJSON(userMessage),
			id,
			'request'
		);
		const responseMessage = createMessageDiv(
			statusIcons.pending,
			id,
			'response'
		);
		messagePairDiv.appendChild(requestMessage);
		messagePairDiv.appendChild(responseMessage);
		chatLog.prepend(messagePairDiv);
	}

	function createMessageDiv(messageContent, id, type) {
		const messageDiv = document.createElement('div');
		messageDiv.classList.add('message', type);

		const content = document.createElement('div');
		if (type === 'response') {
			content.id = id;
		}
		content.classList.add('content');
		content.innerHTML = messageContent;

		const messageIdDiv = document.createElement('div');
		messageIdDiv.classList.add('id');
		messageIdDiv.textContent = `#${id}`;

		messageDiv.appendChild(content);
		messageDiv.appendChild(messageIdDiv);

		return messageDiv;
	}

	if (sendBtn) {
		sendBtn.addEventListener('click', () => {
			sendMessage(input.value);

			helpWindow.classList.remove('show');
			// terminalWindow.classList.toggle('open');
			// terminalWindowHead.querySelector('.plus').classList.toggle('hide');
			// terminalWindowHead.querySelector('.minus').classList.toggle('hide');
			// terminalWindowContent.classList.toggle('hidden');
		});

		input.addEventListener('keypress', (e) => {
			if (e.key === 'Enter' && e.shiftKey) {
				sendMessage(input.value);
				e.preventDefault(); // Prevents default Enter behavior (add new line)
			}
		});
	}

	templateBtns.forEach((btn) => {
		btn.addEventListener('click', () => {
			input.value = JSON.stringify(
				JSON.parse(btn.dataset.template),
				null,
				2
			);
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
		json = json
			.replace(/&/g, '&amp;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;');
		return json.replace(
			/("(\\u[\da-fA-F]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|\d+)/g,
			function (match) {
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
			}
		);
	}

	if (input) {
		input.addEventListener('input', function () {
			try {
				const formattedJSON = JSON.stringify(
					JSON.parse(input.value),
					null,
					2
				);
				input.value = formattedJSON;
			} catch (e) {
				// ignore if the input is not a valid JSON yet
			}
		});
	}

	if (initBtn) {
		initBtn.addEventListener('click', () => {
			if (ready) {
				return;
			}
			const alchemyKey = alchemyKeyInput.value;
			if (!alchemyKey) {
				console.log('Alchemy key is empty');
				return;
			}
			const config = JSON.stringify({
				network: 'mainnet',
				ethereum_url: `http://127.0.0.1:3000/eth-mainnet.g.alchemy.com/v2/${alchemyKey}`,
				starknet_url: `http://127.0.0.1:3000/starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/${alchemyKey}`,
			});
			worker.postMessage(config);
			statusSpan.innerHTML = statusIcons.pending;

			initBtn.innerHTML = statusIcons.pending;
		});
	}

	function timeout(ms, promise) {
		return new Promise((resolve, reject) => {
			const timer = setTimeout(() => {
				reject(new Error('TIMEOUT'));
			}, ms);

			promise
				.then((value) => {
					clearTimeout(timer);
					resolve(value);
				})
				.catch((reason) => {
					clearTimeout(timer);
					reject(reason);
				});
		});
	}

	function checkProxy() {
		if (proxySpan) {
			proxySpan.innerHTML = statusIcons.pending;

			timeout(1000, fetch('http://127.0.0.1:3000/check'))
				.then((response) => response.text())
				.then((response) => {
					console.log('Proxy:', response);
					if (response.trim() === 'ready') {
						proxySpan.innerHTML = statusIcons.ready;
					} else {
						proxySpan.innerHTML = statusIcons.unknown;
					}
				})
				.catch((e) => {
					console.error('Proxy:', e);
					proxySpan.innerHTML = statusIcons.error;
				});
		}
	}

	proxyBtn.addEventListener('click', checkProxy);

	modalOverlay.style.display = 'flex';
	infoBtn.addEventListener('click', () => {
		modalOverlay.style.display = 'flex';
		infoBtn.classList.remove('closed');
	});
	
	helpBtn.addEventListener('click', () => {
		modalOverlay.style.display = 'flex';
		helpBtn.classList.remove('closed');
	});

	document.addEventListener('keydown', (e) => {
		if (e.key === 'Escape') {
			modalOverlay.style.display = 'none';
			infoBtn.classList.add('closed');
			helpBtn.classList.add('closed');
		}
	});

	closeBtn.addEventListener('click', () => {
		modalOverlay.style.display = 'none';
		infoBtn.classList.add('closed');
	});

	terminalWindowHead.addEventListener('click', () => {
		terminalWindow.classList.toggle('open');
		terminalWindowHead.querySelector('.plus').classList.toggle('hide');
		terminalWindowHead.querySelector('.minus').classList.toggle('hide');
		terminalWindowContent.classList.toggle('hidden');
	});

	checkProxy();
});
