#!/bin/bash
source .env
export RUST_LOG
export MINIMAL_LOG_FORMAT
export MINING_PUBKEY
export RUST_LOG_COLOR=false
export RUST_LOG_STYLE=never
export NO_COLOR=1
export TERM=dumb

# Run nockchain, remove color codes and save to file
nockchain --mining-pubkey ${MINING_PUBKEY} --mine 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | tee mining.log

