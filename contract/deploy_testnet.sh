#!/bin/bash

set -e 
sh build.sh
rm -rf .env
rm -rvf neardev

near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/contract.wasm
source neardev/dev-account.env

CONTRACT=$CONTRACT_NAME
OWNER_ID=ower.$CONTRACT

echo "CONTRACT=$CONTRACT" > .env
echo "OWNER_ID=$OWNER_ID" >> .env

set -e

near create-account $OWNER_ID --masterAccount $CONTRACT --initialBalance 5
