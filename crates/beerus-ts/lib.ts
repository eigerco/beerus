import init, { BeerusClient } from "./pkg/index";

export async function createBeerusClient(config: Config): Promise<Client> {
    const wasmData = require("./pkg/index_bg.wasm");
    await init(wasmData);
    return new Client(config);
}

export class Client {
    #client;

    constructor(config: Config) {
        this.#client = new BeerusClient(config.network, config.consensusRpc, config.executionRpc, config.starknetRpc);
    }

    sync_status() {
        return this.#client.sync_status();
    }
}

export type Config = {
    executionRpc: string,
    consensusRpc: string,
    starknetRpc: string,
    network: string,
}

type Request = {
    method: string,
    params: any[],
}
