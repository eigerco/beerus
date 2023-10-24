#!/bin/bash

ETHEREUM_EXECUTION_RPC_URL=$(echo $ETHEREUM_EXECUTION_RPC_URL)

lcp --proxyUrl https://www.lightclientdata.org --port 9001 &
lcp --proxyUrl $ETHEREUM_EXECUTION_RPC_URL --port 9002 &

npx http-server dist
