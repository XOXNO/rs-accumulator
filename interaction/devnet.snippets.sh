ADDRESS=erd1qqqqqqqqqqqqqpgqyxfc4r5fmw2ljcgwxj2nuzv72y9ryvyhah0sgn5vv2
PROXY=https://devnet-gateway.xoxno.com
TOKEN=str:XOXNO-589e09
LIQUID_TOKEN=str:LXOXNO-a00540
LIQUID_SC=erd1qqqqqqqqqqqqqpgq04vxf48vdlr97p3jz73qtxlf4l9p8rezah0s37nzrm
AGGREGATOR_SC=erd1qqqqqqqqqqqqqpgqglqtgt5m50njg6le3calj6hknfuuta2tnrzsuh2qlc
BURN_RATE=1000
SHARE_RATE=3000
PROJECT="./output-docker/accumulator/accumulator.wasm"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} --recall-nonce \
    --arguments ${LIQUID_SC} ${BURN_RATE} ${SHARE_RATE} ${TOKEN} ${LIQUID_TOKEN} ${AGGREGATOR_SC} \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"

    mxpy  contract upgrade ${ADDRESS} --metadata-payable-by-sc --metadata-payable --bytecode=${PROJECT} --recall-nonce \
    --arguments ${LIQUID_SC} ${BURN_RATE} ${SHARE_RATE} ${TOKEN} ${LIQUID_TOKEN} ${AGGREGATOR_SC} \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return
}

verifyContract() {
    mxpy --verbose contract verify "${ADDRESS}"  \
    --packaged-src=./output-docker/accumulator/accumulator-0.0.0.source.json --verifier-url="https://devnet-play-api.multiversx.com" \
    --docker-image="multiversx/sdk-rust-contract-builder:v11.0.0" --ledger --ledger-account-index=0 --ledger-address-index=0  || return 
}