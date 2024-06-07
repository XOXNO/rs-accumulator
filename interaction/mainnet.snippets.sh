ADDRESS=erd1qqqqqqqqqqqqqpgqyxfc4r5fmw2ljcgwxj2nuzv72y9ryvyhah0sgn5vv2
PROXY=https://devnet-gateway.xoxno.com
TOKEN=str:XOXNO-589e09
LIQUID_TOKEN=str:LXOXNO-a00540
LIQUID_SC=erd1qqqqqqqqqqqqqpgq04vxf48vdlr97p3jz73qtxlf4l9p8rezah0s37nzrm
AGGREGATOR_SC=erd1qqqqqqqqqqqqqpgqh96hhj42huhe47j3jerlec7ndhw75gy72gesy7w2d6
BURN_RATE=3000
SHARE_RATE=4000
PROJECT="/Users/mihaieremia/GitHub/rs-accumulator/output/accumulator.wasm"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} --recall-nonce \
    --arguments ${LIQUID_SC} ${BURN_RATE} ${SHARE_RATE} ${TOKEN} ${LIQUID_TOKEN} ${AGGREGATOR_SC} \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --guardian erd1789rujqce0ya72k03h2jp3pgqf3vdtt0e8740tndfj0jstx3w78qxcewr8 \
    --guardian-service-url https://tools.multiversx.com/guardian \
    --guardian-2fa-code 730372 --version 2 --options 2 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"

    mxpy  contract upgrade ${ADDRESS} --bytecode=${PROJECT} --recall-nonce \
    --arguments ${LIQUID_SC} ${BURN_RATE} ${SHARE_RATE} ${TOKEN} ${LIQUID_TOKEN} ${AGGREGATOR_SC} \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return
}
