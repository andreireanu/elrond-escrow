PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/elrond-escrow/wallets/alice.pem"
WALLET_BOB="${PWD}/elrond-escrow/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgq8qtky6y4l6pljtgf75alj0tf748h7u227wpqe9t0wc"
TREASURY_WALLET="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
TREASURY_WALLET_HEX="0x$(erdpy wallet bech32 --decode ${TREASURY_WALLET})"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(erdpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(erdpy wallet bech32 --decode ${MARTA_ADDRESS})"

# PAST TIMESTAMP = 1652438788
# FUTURE TIMESTAMP = 1683974788
START_TIMESTAMP=1652438788

#### TEST TOKENS:
## ESC1-492f2b
## ESC2-83fea1

deploy() {
 erdpy contract deploy --chain="D" \
    --outfile="elrond-escrow/interaction/deploy-devnet.interaction.json" \
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
    --gas-limit=50000000 \
    --arguments ${START_TIMESTAMP}
}

######## SET START TIMESTAMP

setStartTimestamp() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=2500000 \
    --proxy=${PROXY} \
    --function="setStartTimestamp" \
    --arguments ${START_TIMESTAMP}
}

######## DISALLOWED TOKENS FOR ESCROW

## ESC1-492f2b
## ESC2-83fea1

TOKEN_ID="ESC2-83fea1"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
 
addDisallowedToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=6000000 \
    --proxy=${PROXY} \
    --function="addDisallowedToken" \
    --arguments ${TOKEN_ID_HEX}
}

removeDisallowedToken() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem=${WALLET_ALICE} \
    --gas-limit=6000000 \
    --proxy=${PROXY} \
    --function="removeDisallowedToken" \
    --arguments ${TOKEN_ID_HEX}
}
 
getDisallowedTokens() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getDisallowedTokens" > ${PWD}/elrond-escrow/interaction/getDisallowedTokens.json

    ESDTS_HEX=$(erdpy data parse --file="${PWD}/elrond-escrow/interaction/getDisallowedTokens.json" --expression="data[0]['hex']")
    echo ${ESDTS_HEX}
    ESDTS=$(echo ${ESDTS_HEX} | xxd -r -p)
    echo "ESDTS: ${ESDTS}"
    echo -e "\nESDTS: ${ESDTS}" >> ${PWD}/elrond-escrow/interaction/getDisallowedTokens.json
}

######## GET DATA STORED WITH SENDER WALLET KEY
 
getSendData() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSendData" > ${PWD}/elrond-escrow/interaction/getSendData.json \
    --arguments ${ALICE_ADDRESS_HEXX} 
    python3  ${PWD}/elrond-escrow/interaction/format_escrow_data.py send ${ALICE_ADDRESS}
    }
 
######## GET DATA STORED WITH RECEIVER WALLET KEY
 
getReceiveData() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getReceiveData" > ${PWD}/elrond-escrow/interaction/getReceiveData.json \
    --arguments ${BOB_ADDRESS_HEXX}
    python3  ${PWD}/elrond-escrow/interaction/format_escrow_data.py receive ${BOB_ADDRESS}
    }

######## CLEAR DATABASE
 
clearData() {
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
 
######## ESCROW

# TOKEN_TO="ESC1-492f2b"
# TOKEN_FROM="ESC2-83fea1"

TOKEN_TO="ESC1-492f2b"
TOKEN_TO_HEX="$(echo -n ${TOKEN_TO} | xxd -p -u | tr -d '\n')"

VALUE_TO=4.9
DECIMALS_TO=18
POWER_TO=$((10**${DECIMALS_TO}))
AMOUNT_TO=$( printf "%.0f" $(echo "${VALUE_TO} * ${POWER_TO}" | bc) ) 
AMOUNT_TO_HEX=$(python3 ${PWD}/elrond-escrow/interaction/to_hex.py ${AMOUNT_TO})
 
TOKEN_FROM="ESC2-83fea1"
TOKEN_FROM_HEX="$(echo -n ${TOKEN_FROM} | xxd -p -u | tr -d '\n')"

VALUE_FROM=4.2 
DECIMALS_FROM=18
POWER_FROM=$((10**${DECIMALS_FROM}))
AMOUNT_FROM=$( printf "%.0f" $(echo "${VALUE_FROM} * ${POWER_FROM}" | bc) ) 
AMOUNT_FROM_HEX=$(python3  ${PWD}/elrond-escrow/interaction/to_hex.py ${AMOUNT_FROM})
 
ADD_OFFER="addOffer"
ADD_OFFER_HEX="$(echo -n ${ADD_OFFER} | xxd -p -u | tr -d '\n')"

addOffer() {
    erdpy --verbose tx new \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=10000000 \
    --receiver=${CONTRACT_ADDRESS} \
    --data="ESDTTransfer@${TOKEN_TO_HEX}@${AMOUNT_TO_HEX}@${ADD_OFFER_HEX}@${TOKEN_TO_HEX}@${AMOUNT_TO_HEX}@${TOKEN_FROM_HEX}@${AMOUNT_FROM_HEX}@${BOB_ADDRESS_HEX}"  
}

ACCEPT_OFFER="acceptOffer"
ACCEPT_OFFER_HEX="$(echo -n ${ACCEPT_OFFER} | xxd -p -u | tr -d '\n')"

acceptOffer() {
    erdpy --verbose tx new \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_BOB} \
    --recall-nonce \
    --gas-limit=10000000 \
    --receiver=${CONTRACT_ADDRESS} \
    --data="ESDTTransfer@${TOKEN_FROM_HEX}@${AMOUNT_FROM_HEX}@${ACCEPT_OFFER_HEX}@${TOKEN_FROM_HEX}@${AMOUNT_FROM_HEX}@${TOKEN_TO_HEX}@${AMOUNT_TO_HEX}@${ALICE_ADDRESS_HEX}"  
}
 
######## REMOVE OFFER

removeOffer() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=8000000 \
    --function="removeOffer" \
    --arguments "str:"$TOKEN_TO $AMOUNT_TO "str:"$TOKEN_FROM $AMOUNT_FROM $BOB_ADDRESS
    }   

######## SEND TOKEN

send() {
    erdpy --verbose tx new \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --pem=${WALLET_ALICE} \
    --recall-nonce \
    --gas-limit=6000000 \
    --receiver=${BOB_ADDRESS} \
    --data="ESDTTransfer@${TOKEN_FROM_HEX}@${AMOUNT_FROM_HEX}"  
}
