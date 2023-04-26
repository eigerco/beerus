POST http://0.0.0.0:3030
Content-Type: application/json
{
    "id": 1,
    "jsonrpc": "2.0",
    "params": [
        "0x4D73AdB72bC3DD368966edD0f0b2148401A178E2",
        "latest"
    ],
    "method": "eth_getCode"
}

HTTP 200
[Asserts]
# jsonpath "$.error" not exists
# jsonpath "$.result" matches /^0x[A-Fa-f0-9]+$/
