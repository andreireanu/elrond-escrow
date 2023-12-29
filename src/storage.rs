elrond_wasm::imports!();
elrond_wasm::derive_imports!();
 
// STORAGE

use crate::EscrowWalletFormat;

#[elrond_wasm::module]
pub trait StorageModule {
    /// START TIMESTAMP
    #[storage_mapper("esdt_mapper")]
    fn start_timestamp(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setStartTimestamp)]
    fn set_start_timestamp(&self, start_timestamp: u64) {
        self.start_timestamp().set(&start_timestamp)
    }

    #[only_owner]
    #[endpoint(getStartTimestamp)]
    fn get_start_timestamp(&self) {
        self.start_timestamp();
    }

    /// SUCCESFUL ESCROW TRANSACTIONS
    #[storage_mapper("escrowsNo")]
    fn escrows_no(&self) -> SingleValueMapper<i32>;

    #[view(getEscrowsNo)]
    fn get_escrows_no(&self) -> SingleValueMapper<i32> {
        self.escrows_no()
    }

    /// DISSALOWED ESCROW TOKENS
    #[view(getDisallowedTokens)]
    #[storage_mapper("dissalowed_tokens")]
    fn dissalowed_tokens(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[only_owner]
    #[endpoint(addDisallowedToken)]
    fn add_dissallowd_tokens(&self, token_id: TokenIdentifier) {
        self.dissalowed_tokens().insert(token_id);
    }

    #[only_owner]
    #[endpoint(removeDisallowedToken)]
    fn remove_dissallowd_tokens(&self, token_id: TokenIdentifier) {
        self.dissalowed_tokens().swap_remove(&token_id);
    }

    /// ESCROW DATA

    /// DATA STORED WITH SENDER WALLET KEY
    #[storage_mapper("send_data")]
    fn send_data(
        &self,
    ) -> MapStorageMapper<ManagedAddress, UnorderedSetMapper<EscrowWalletFormat<Self::Api>>>;

    #[view(getSendData)]
    fn get_send_data(
        &self,
        address: &ManagedAddress,
    ) -> UnorderedSetMapper<EscrowWalletFormat<Self::Api>> {
        self.send_data().get(address).unwrap()
    }

    /// DATA STORED WITH RECEIVER WALLET KEY
    #[storage_mapper("receive_data")]
    fn receive_data(
        &self,
    ) -> MapStorageMapper<ManagedAddress, UnorderedSetMapper<EscrowWalletFormat<Self::Api>>>;

    #[view(getReceiveData)]
    fn get_receive_data(
        &self,
        address: &ManagedAddress,
    ) -> UnorderedSetMapper<EscrowWalletFormat<Self::Api>> {
        self.receive_data().get(address).unwrap()
    }
}
