ADDRESS=erd1qqqqqqqqqqqqqpgq8538ku69p97lq4eug75y8d6g6yfwhd7c45qs4zvejt
PROXY=https://gateway.xoxno.com
TOKEN=str:XOXNO-c1293a
LIQUID_TOKEN=str:LXOXNO-0eb983
LIQUID_SC=erd1qqqqqqqqqqqqqpgqs5w0wfmf5gw7qae82upgu26cpk2ug8l245qszu3dxf
AGGREGATOR_SC=erd1qqqqqqqqqqqqqpgq5rf2sppxk2xu4m0pkmugw2es4gak3rgjah0sxvajva
BURN_RATE=1000
SHARE_RATE=3000
PROJECT="./output-docker/accumulator/accumulator.wasm"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} \
    --arguments ${LIQUID_SC} ${BURN_RATE} ${SHARE_RATE} ${TOKEN} ${LIQUID_TOKEN} ${AGGREGATOR_SC} \
    --ledger \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"

    mxpy  contract upgrade ${ADDRESS} --metadata-payable-by-sc --metadata-payable --bytecode=${PROJECT} \
    --arguments ${LIQUID_SC} ${BURN_RATE} ${SHARE_RATE} ${TOKEN} ${LIQUID_TOKEN} ${AGGREGATOR_SC} \
    --ledger \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return
}

verifyContract() {
    mxpy --verbose contract verify "${ADDRESS}"  \
    --packaged-src=./output-docker/accumulator/accumulator-0.0.0.source.json --verifier-url="https://play-api.multiversx.com" \
    --docker-image="multiversx/sdk-rust-contract-builder:v11.0.0" --ledger || return 
}

buildDocker() {
    mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v11.0.0"
}
