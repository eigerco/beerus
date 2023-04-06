POST http://0.0.0.0:3030
Content-Type: application/json
{
  "jsonrpc": "2.0",
  "method": "starknet_addInvokeTransaction",
  "params": {
    "invoke_transaction": {
      "contract_address": "0x23371b227eaecd8e8920cd429d2cd0f3fee6abaacca08d3ab82a7cdd",
      "calldata": [
        "0x1",
        "0x677bb1cdc050e8d63855e8743ab6e09179138def390676cc03c484daf112ba1",
        "0x362398bec32bc0ebb411203221a35a0301193a96f317ebe5e40be9f60d15320",
        "0x0",
        "0x1",
        "0x1",
        "0x2b",
        "0x0"
      ],
      "entry_point_selector": "0x15d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad"
    },
    "signature": [
      "3557065757165699682249469970267166698995647077461960906176449260016084767701",
      "3202126414680946801789588986259466145787792017299869598314522555275920413944"
    ],
    "max_fee": "0x4f388496839",
    "version": "0x0"
  },
  "id": 1
}
