import("./pkg/").then((lib) => {
    lib.BeerusClient.new(
        "mainnet",
        "http://localhost:9001/proxy",
        "http://localhost:9002/proxy",
        "http://localhost:9545",
        ).then(client => {
        let status = client.get_sync_status();
        console.log("Beerus Initialized: ", status);

        client.get_starknet_state_root().then(state_root => {
            console.log("StarkNet State Root: ", state_root);
        });
    });
});
