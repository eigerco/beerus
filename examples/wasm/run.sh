#!/bin/bash

ETH_EXECUTION_RPC=$(echo $ETH_EXECUTION_RPC)

lcp --proxyUrl https://www.lightclientdata.org --port 9001 &
lcp --proxyUrl $ETH_EXECUTION_RPC --port 9002 &

npx http-server dist
