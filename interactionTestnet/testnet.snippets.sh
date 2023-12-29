PROXY=https://testnet-api.multiversx.com
CHAIN_ID="T"
WALLET_ALICE="${PWD}/elrond-escrow/wallets/alice.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgq5tjds4m87v9r3wchnf3e3zdgdwhgf33j7wpq2m0t3r"
 
START_TIMESTAMP=1664632708

deploy() {
 erdpy contract deploy --chain=${CHAIN_ID} \
    --outfile="elrond-escrow/interactionTestnet/testnet.interaction.json" \
    --project=elrond-escrow \
    --pem="elrond-escrow/wallets/alice.pem" \
    --gas-limit=60000000 \
    --proxy=${PROXY} \
    --recall-nonce \
    --send \
    --arguments ${START_TIMESTAMP}
}
  
upgrade() {
    erdpy contract upgrade ${CONTRACT_ADDRESS} \
    --project=elrond-escrow \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --send \
    --metadata-payable \
    --outfile="elrond-escrow/interaction/upgrade.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --gas-limit=60000000 \
    --arguments ${START_TIMESTAMP}
}

ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"

getSendData() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSendData" > ${PWD}/elrond-escrow/interactionTestnet/getSendData.json \
    --arguments ${ALICE_ADDRESS_HEXX} 
    python3  ${PWD}/elrond-escrow/interactionTestnet/format_escrow_data.py send ${BOB_ADDRESS}
    }

getReceiveData() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReceiveData" > ${PWD}/elrond-escrow/interactionTestnet/getReceiveData.json \
    --arguments ${BOB_ADDRESS_HEXX} 
    python3  ${PWD}/elrond-escrow/interactionTestnet/format_escrow_data.py receive ${BOB_ADDRESS}
    }    
 

clearDataAlice() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=8000000 \
    --function="clear" \
    --arguments ${ALICE_ADDRESS_HEXX}
    }  

clearDataBob() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=8000000 \
    --function="clear" \
    --arguments ${BOB_ADDRESS_HEXX}
    }  