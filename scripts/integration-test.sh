#!/bin/bash

# Executes end-to-end tests.
#
# It is meant to be run in the CI on a daily basis and provide a report of the
# endpoint health by:
# - execute every .hurl file in the examples/ directory
# - output an inline summary in the console
# - exit with an error code in case any hurl test fails

echo "wait for beerus to sync and listen..."
while ! timeout 1 bash -c "echo > /dev/tcp/localhost/3030" 2> /dev/null; do sleep 2; done
echo "beerus in sync..."

[ -z "$1" ] && network="mainnet" || network=$1
echo "running test for $network..."

FAILED=0
for request in $(find examples/rpc/$network/ -name "*Nonce*"); do
    hurl --test --max-time=50 --error-format=long $request
    if [ $? -ne 0 ]; then
        echo "FAILED REQUEST - $request"
        FAILED=$((FAILED + 1))
    fi
done

if ((FAILED > 0)); then
    echo "$FAILED FAILED REQUESTS"
    exit 1
fi
