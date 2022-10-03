# source: https://stackoverflow.com/questions/69595384/erdpy-token-issuance-transaction-fails-with-code-internal-issue
# Script outcome: The receiver address sends the tokens to the pem address wallet


from erdpy.accounts import Account, Address
from erdpy.proxy import ElrondProxy
from erdpy.transactions import BunchOfTransactions
from erdpy.transactions import Transaction
from erdpy.wallet import signing
import binascii as bs


TOKEN_NAME = b"PACTE"
TOKEN_SYMBOL = b"PACTE"

DECIMALS = 6
SUPPLY = 1000000000 * 10**DECIMALS


def hex_string(s: str) -> str:
    assert type(s) == bytes, "Make sure everything is bytes data or utf-8 encoded"
    return bs.hexlify(s).decode("ascii")


def hex_int(i: int) -> str:
    assert type(i) == int, "Make sure everything is bytes data or utf-8 encoded"
    temp = hex(i)[2:]
    if len(temp) % 2 == 1:
        temp = "0" + temp
    return temp


proxy = ElrondProxy("https://testnet-gateway.elrond.com")
sender = Account(pem_file="../wallets/alice.pem")
sender.sync_nonce(proxy)

tx = Transaction()
tx.nonce = sender.nonce
tx.value = str(int(0.05 * 10**18))  # 0.05 EGLD, as required for issuing a token according to the documentation
tx.sender = sender.address.bech32()
# System contract address to issue out the new token as per
# https://docs.elrond.com/developers/esdt-tokens/#issuance-of-fungible-esdt-tokens
tx.receiver = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"
tx.gasPrice = 1000000000
tx.gasLimit = 60000000
tx.data = f"issue" \
          f"@{hex_string(TOKEN_NAME)}" \
          f"@{hex_string(TOKEN_SYMBOL)}" \
          f"@{hex_int(SUPPLY)}" \
          f"@{hex_int(DECIMALS)}" 

tx.chainID = "T"  # For devnet https://devnet-gateway.elrond.com/network/config
tx.version = 1

#New version for generating a signature:
tx.signature = sender.sign_transaction(tx)

#Old version which is now obsolete:
#tx.signature = signing.sign_transaction(tx, sender)

tx.send(proxy)